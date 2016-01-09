/*
 * This file generated automatically from xf86vidmode.xml by r_client.py.
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
use ffi::xf86vidmode::*;
use std::option::Option;
use std::iter::Iterator;

pub type Syncrange = xcb_xf86vidmode_syncrange_t;

pub type SyncrangeIterator = xcb_xf86vidmode_syncrange_iterator_t;

pub type DotclockIterator = xcb_xf86vidmode_dotclock_iterator_t;


pub type xcb_xf86vidmode_mode_flag_t = c_uint;//{
    pub const XCB_XF86VIDMODE_MODE_FLAG_POSITIVE_H_SYNC : xcb_xf86vidmode_mode_flag_t = 1;
    pub const XCB_XF86VIDMODE_MODE_FLAG_NEGATIVE_H_SYNC : xcb_xf86vidmode_mode_flag_t = 2;
    pub const XCB_XF86VIDMODE_MODE_FLAG_POSITIVE_V_SYNC : xcb_xf86vidmode_mode_flag_t = 4;
    pub const XCB_XF86VIDMODE_MODE_FLAG_NEGATIVE_V_SYNC : xcb_xf86vidmode_mode_flag_t = 8;
    pub const XCB_XF86VIDMODE_MODE_FLAG_INTERLACE : xcb_xf86vidmode_mode_flag_t = 16;
    pub const XCB_XF86VIDMODE_MODE_FLAG_COMPOSITE_SYNC : xcb_xf86vidmode_mode_flag_t = 32;
    pub const XCB_XF86VIDMODE_MODE_FLAG_POSITIVE_C_SYNC : xcb_xf86vidmode_mode_flag_t = 64;
    pub const XCB_XF86VIDMODE_MODE_FLAG_NEGATIVE_C_SYNC : xcb_xf86vidmode_mode_flag_t = 128;
    pub const XCB_XF86VIDMODE_MODE_FLAG_H_SKEW : xcb_xf86vidmode_mode_flag_t = 256;
    pub const XCB_XF86VIDMODE_MODE_FLAG_BROADCAST : xcb_xf86vidmode_mode_flag_t = 512;
    pub const XCB_XF86VIDMODE_MODE_FLAG_PIXMUX : xcb_xf86vidmode_mode_flag_t = 1024;
    pub const XCB_XF86VIDMODE_MODE_FLAG_DOUBLE_CLOCK : xcb_xf86vidmode_mode_flag_t = 2048;
    pub const XCB_XF86VIDMODE_MODE_FLAG_HALF_CLOCK : xcb_xf86vidmode_mode_flag_t = 4096;
//}

pub type xcb_xf86vidmode_clock_flag_t = c_uint;//{
    pub const XCB_XF86VIDMODE_CLOCK_FLAG_PROGRAMABLE : xcb_xf86vidmode_clock_flag_t = 1;
//}

pub type xcb_xf86vidmode_permission_t = c_uint;//{
    pub const XCB_XF86VIDMODE_PERMISSION_READ : xcb_xf86vidmode_permission_t = 1;
    pub const XCB_XF86VIDMODE_PERMISSION_WRITE : xcb_xf86vidmode_permission_t = 2;
//}
pub struct ModeInfo {pub base : base::Struct<xcb_xf86vidmode_mode_info_t> }

pub type ModeInfoIterator = xcb_xf86vidmode_mode_info_iterator_t;

pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, xcb_xf86vidmode_query_version_cookie_t> }

/** Opcode for xcb_xf86vidmode_query_version. */
pub const XCB_XF86VIDMODE_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<xcb_xf86vidmode_query_version_reply_t> }
fn mk_reply_xcb_xf86vidmode_query_version_reply_t(reply:*mut xcb_xf86vidmode_query_version_reply_t) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
pub struct  GetModeLineCookie<'s> { pub base : base::Cookie<'s, xcb_xf86vidmode_get_mode_line_cookie_t> }

/** Opcode for xcb_xf86vidmode_get_mode_line. */
pub const XCB_XF86VIDMODE_GET_MODE_LINE : u8 = 1;
/** Opcode for xcb_xf86vidmode_mod_mode_line. */
pub const XCB_XF86VIDMODE_MOD_MODE_LINE : u8 = 2;
/** Opcode for xcb_xf86vidmode_switch_mode. */
pub const XCB_XF86VIDMODE_SWITCH_MODE : u8 = 3;
pub struct  GetMonitorCookie<'s> { pub base : base::Cookie<'s, xcb_xf86vidmode_get_monitor_cookie_t> }

/** Opcode for xcb_xf86vidmode_get_monitor. */
pub const XCB_XF86VIDMODE_GET_MONITOR : u8 = 4;
/** Opcode for xcb_xf86vidmode_lock_mode_switch. */
pub const XCB_XF86VIDMODE_LOCK_MODE_SWITCH : u8 = 5;
pub struct  GetAllModeLinesCookie<'s> { pub base : base::Cookie<'s, xcb_xf86vidmode_get_all_mode_lines_cookie_t> }

/** Opcode for xcb_xf86vidmode_get_all_mode_lines. */
pub const XCB_XF86VIDMODE_GET_ALL_MODE_LINES : u8 = 6;
/** Opcode for xcb_xf86vidmode_add_mode_line. */
pub const XCB_XF86VIDMODE_ADD_MODE_LINE : u8 = 7;
/** Opcode for xcb_xf86vidmode_delete_mode_line. */
pub const XCB_XF86VIDMODE_DELETE_MODE_LINE : u8 = 8;
pub struct  ValidateModeLineCookie<'s> { pub base : base::Cookie<'s, xcb_xf86vidmode_validate_mode_line_cookie_t> }

/** Opcode for xcb_xf86vidmode_validate_mode_line. */
pub const XCB_XF86VIDMODE_VALIDATE_MODE_LINE : u8 = 9;
pub struct ValidateModeLineReply { base:  base::Reply<xcb_xf86vidmode_validate_mode_line_reply_t> }
fn mk_reply_xcb_xf86vidmode_validate_mode_line_reply_t(reply:*mut xcb_xf86vidmode_validate_mode_line_reply_t) -> ValidateModeLineReply { ValidateModeLineReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_xf86vidmode_switch_to_mode. */
pub const XCB_XF86VIDMODE_SWITCH_TO_MODE : u8 = 10;
pub struct  GetViewPortCookie<'s> { pub base : base::Cookie<'s, xcb_xf86vidmode_get_view_port_cookie_t> }

/** Opcode for xcb_xf86vidmode_get_view_port. */
pub const XCB_XF86VIDMODE_GET_VIEW_PORT : u8 = 11;
pub struct GetViewPortReply { base:  base::Reply<xcb_xf86vidmode_get_view_port_reply_t> }
fn mk_reply_xcb_xf86vidmode_get_view_port_reply_t(reply:*mut xcb_xf86vidmode_get_view_port_reply_t) -> GetViewPortReply { GetViewPortReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_xf86vidmode_set_view_port. */
pub const XCB_XF86VIDMODE_SET_VIEW_PORT : u8 = 12;
pub struct  GetDotClocksCookie<'s> { pub base : base::Cookie<'s, xcb_xf86vidmode_get_dot_clocks_cookie_t> }

/** Opcode for xcb_xf86vidmode_get_dot_clocks. */
pub const XCB_XF86VIDMODE_GET_DOT_CLOCKS : u8 = 13;
/** Opcode for xcb_xf86vidmode_set_client_version. */
pub const XCB_XF86VIDMODE_SET_CLIENT_VERSION : u8 = 14;
/** Opcode for xcb_xf86vidmode_set_gamma. */
pub const XCB_XF86VIDMODE_SET_GAMMA : u8 = 15;
pub struct  GetGammaCookie<'s> { pub base : base::Cookie<'s, xcb_xf86vidmode_get_gamma_cookie_t> }

/** Opcode for xcb_xf86vidmode_get_gamma. */
pub const XCB_XF86VIDMODE_GET_GAMMA : u8 = 16;
pub struct GetGammaReply { base:  base::Reply<xcb_xf86vidmode_get_gamma_reply_t> }
fn mk_reply_xcb_xf86vidmode_get_gamma_reply_t(reply:*mut xcb_xf86vidmode_get_gamma_reply_t) -> GetGammaReply { GetGammaReply { base : base::mk_reply(reply) } }
pub struct  GetGammaRampCookie<'s> { pub base : base::Cookie<'s, xcb_xf86vidmode_get_gamma_ramp_cookie_t> }

/** Opcode for xcb_xf86vidmode_get_gamma_ramp. */
pub const XCB_XF86VIDMODE_GET_GAMMA_RAMP : u8 = 17;
/** Opcode for xcb_xf86vidmode_set_gamma_ramp. */
pub const XCB_XF86VIDMODE_SET_GAMMA_RAMP : u8 = 18;
pub struct  GetGammaRampSizeCookie<'s> { pub base : base::Cookie<'s, xcb_xf86vidmode_get_gamma_ramp_size_cookie_t> }

/** Opcode for xcb_xf86vidmode_get_gamma_ramp_size. */
pub const XCB_XF86VIDMODE_GET_GAMMA_RAMP_SIZE : u8 = 19;
pub struct GetGammaRampSizeReply { base:  base::Reply<xcb_xf86vidmode_get_gamma_ramp_size_reply_t> }
fn mk_reply_xcb_xf86vidmode_get_gamma_ramp_size_reply_t(reply:*mut xcb_xf86vidmode_get_gamma_ramp_size_reply_t) -> GetGammaRampSizeReply { GetGammaRampSizeReply { base : base::mk_reply(reply) } }
pub struct  GetPermissionsCookie<'s> { pub base : base::Cookie<'s, xcb_xf86vidmode_get_permissions_cookie_t> }

/** Opcode for xcb_xf86vidmode_get_permissions. */
pub const XCB_XF86VIDMODE_GET_PERMISSIONS : u8 = 20;
pub struct GetPermissionsReply { base:  base::Reply<xcb_xf86vidmode_get_permissions_reply_t> }
fn mk_reply_xcb_xf86vidmode_get_permissions_reply_t(reply:*mut xcb_xf86vidmode_get_permissions_reply_t) -> GetPermissionsReply { GetPermissionsReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_xf86vidmode_bad_clock. */
pub const XCB_XF86VIDMODE_BAD_CLOCK : u8 = 0;
pub struct BadClockError { pub base : base::Error<xcb_xf86vidmode_bad_clock_error_t> }
/** Opcode for xcb_xf86vidmode_bad_h_timings. */
pub const XCB_XF86VIDMODE_BAD_H_TIMINGS : u8 = 1;
pub struct BadHTimingsError { pub base : base::Error<xcb_xf86vidmode_bad_h_timings_error_t> }
/** Opcode for xcb_xf86vidmode_bad_v_timings. */
pub const XCB_XF86VIDMODE_BAD_V_TIMINGS : u8 = 2;
pub struct BadVTimingsError { pub base : base::Error<xcb_xf86vidmode_bad_v_timings_error_t> }
/** Opcode for xcb_xf86vidmode_mode_unsuitable. */
pub const XCB_XF86VIDMODE_MODE_UNSUITABLE : u8 = 3;
pub struct ModeUnsuitableError { pub base : base::Error<xcb_xf86vidmode_mode_unsuitable_error_t> }
/** Opcode for xcb_xf86vidmode_extension_disabled. */
pub const XCB_XF86VIDMODE_EXTENSION_DISABLED : u8 = 4;
pub struct ExtensionDisabledError { pub base : base::Error<xcb_xf86vidmode_extension_disabled_error_t> }
/** Opcode for xcb_xf86vidmode_client_not_local. */
pub const XCB_XF86VIDMODE_CLIENT_NOT_LOCAL : u8 = 5;
pub struct ClientNotLocalError { pub base : base::Error<xcb_xf86vidmode_client_not_local_error_t> }
/** Opcode for xcb_xf86vidmode_zoom_locked. */
pub const XCB_XF86VIDMODE_ZOOM_LOCKED : u8 = 6;
pub struct ZoomLockedError { pub base : base::Error<xcb_xf86vidmode_zoom_locked_error_t> }


impl Iterator for SyncrangeIterator {
    type Item = Syncrange;
    fn next(&mut self) -> Option<Syncrange> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xf86vidmode_syncrange_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xf86vidmode_syncrange_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Dotclock = xcb_xf86vidmode_dotclock_t;


impl Iterator for DotclockIterator {
    type Item = Dotclock;
    fn next(&mut self) -> Option<Dotclock> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xf86vidmode_dotclock_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xf86vidmode_dotclock_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl ModeInfo {
  pub fn dotclock(&mut self) -> Dotclock {
    unsafe { accessor!(dotclock -> Dotclock, self.base.strct) }
  }

  pub fn hdisplay(&mut self) -> u16 {
    unsafe { accessor!(hdisplay -> u16, self.base.strct) }
  }

  pub fn hsyncstart(&mut self) -> u16 {
    unsafe { accessor!(hsyncstart -> u16, self.base.strct) }
  }

  pub fn hsyncend(&mut self) -> u16 {
    unsafe { accessor!(hsyncend -> u16, self.base.strct) }
  }

  pub fn htotal(&mut self) -> u16 {
    unsafe { accessor!(htotal -> u16, self.base.strct) }
  }

  pub fn hskew(&mut self) -> u32 {
    unsafe { accessor!(hskew -> u32, self.base.strct) }
  }

  pub fn vdisplay(&mut self) -> u16 {
    unsafe { accessor!(vdisplay -> u16, self.base.strct) }
  }

  pub fn vsyncstart(&mut self) -> u16 {
    unsafe { accessor!(vsyncstart -> u16, self.base.strct) }
  }

  pub fn vsyncend(&mut self) -> u16 {
    unsafe { accessor!(vsyncend -> u16, self.base.strct) }
  }

  pub fn vtotal(&mut self) -> u16 {
    unsafe { accessor!(vtotal -> u16, self.base.strct) }
  }

  pub fn flags(&mut self) -> u32 {
    unsafe { accessor!(flags -> u32, self.base.strct) }
  }

  pub fn privsize(&mut self) -> u32 {
    unsafe { accessor!(privsize -> u32, self.base.strct) }
  }

}

impl Iterator for ModeInfoIterator {
    type Item = ModeInfo;
    fn next(&mut self) -> Option<ModeInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xf86vidmode_mode_info_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xf86vidmode_mode_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn QueryVersion<'r> (c : &'r Connection) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_query_version(c.get_raw_conn());
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_query_version_unchecked(c.get_raw_conn());
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
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_xcb_xf86vidmode_query_version_reply_t, QueryVersionReply, xcb_xf86vidmode_query_version_reply);

pub struct GetModeLineReply { base:  base::Reply<xcb_xf86vidmode_get_mode_line_reply_t> }
fn mk_reply_xcb_xf86vidmode_get_mode_line_reply_t(reply:*mut xcb_xf86vidmode_get_mode_line_reply_t) -> GetModeLineReply { GetModeLineReply { base : base::mk_reply(reply) } }
pub fn GetModeLine<'r> (c : &'r Connection,
                    screen : u16) -> GetModeLineCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_mode_line(c.get_raw_conn(),
        screen as u16); //1
    GetModeLineCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetModeLineUnchecked<'r> (c : &'r Connection,
                             screen : u16) -> GetModeLineCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_mode_line_unchecked(c.get_raw_conn(),
        screen as u16); //1
    GetModeLineCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetModeLineReply {
  pub fn dotclock(&mut self) -> Dotclock {
    unsafe { accessor!(dotclock -> Dotclock, (*self.base.reply)) }
  }

  pub fn hdisplay(&mut self) -> u16 {
    unsafe { accessor!(hdisplay -> u16, (*self.base.reply)) }
  }

  pub fn hsyncstart(&mut self) -> u16 {
    unsafe { accessor!(hsyncstart -> u16, (*self.base.reply)) }
  }

  pub fn hsyncend(&mut self) -> u16 {
    unsafe { accessor!(hsyncend -> u16, (*self.base.reply)) }
  }

  pub fn htotal(&mut self) -> u16 {
    unsafe { accessor!(htotal -> u16, (*self.base.reply)) }
  }

  pub fn hskew(&mut self) -> u16 {
    unsafe { accessor!(hskew -> u16, (*self.base.reply)) }
  }

  pub fn vdisplay(&mut self) -> u16 {
    unsafe { accessor!(vdisplay -> u16, (*self.base.reply)) }
  }

  pub fn vsyncstart(&mut self) -> u16 {
    unsafe { accessor!(vsyncstart -> u16, (*self.base.reply)) }
  }

  pub fn vsyncend(&mut self) -> u16 {
    unsafe { accessor!(vsyncend -> u16, (*self.base.reply)) }
  }

  pub fn vtotal(&mut self) -> u16 {
    unsafe { accessor!(vtotal -> u16, (*self.base.reply)) }
  }

  pub fn flags(&mut self) -> u32 {
    unsafe { accessor!(flags -> u32, (*self.base.reply)) }
  }

  pub fn private(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_xf86vidmode_get_mode_line_private_length, xcb_xf86vidmode_get_mode_line_private, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetModeLineCookie<'s>, mk_reply_xcb_xf86vidmode_get_mode_line_reply_t, GetModeLineReply, xcb_xf86vidmode_get_mode_line_reply);

pub fn ModModeLineChecked<'r> (c : &'r Connection,
                           screen : u32,
                           hdisplay : u16,
                           hsyncstart : u16,
                           hsyncend : u16,
                           htotal : u16,
                           hskew : u16,
                           vdisplay : u16,
                           vsyncstart : u16,
                           vsyncend : u16,
                           vtotal : u16,
                           flags : u32,
                           private : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let private_len = private.len();
    let private_ptr = private.as_ptr();
    let cookie = xcb_xf86vidmode_mod_mode_line_checked(c.get_raw_conn(),
        screen as u32, //1
        hdisplay as u16, //2
        hsyncstart as u16, //3
        hsyncend as u16, //4
        htotal as u16, //5
        hskew as u16, //6
        vdisplay as u16, //7
        vsyncstart as u16, //8
        vsyncend as u16, //9
        vtotal as u16, //10
        flags as u32, //11
        private_len as u32, //12
        private_ptr as *mut u8); //13
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ModModeLine<'r> (c : &'r Connection,
                    screen : u32,
                    hdisplay : u16,
                    hsyncstart : u16,
                    hsyncend : u16,
                    htotal : u16,
                    hskew : u16,
                    vdisplay : u16,
                    vsyncstart : u16,
                    vsyncend : u16,
                    vtotal : u16,
                    flags : u32,
                    private : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let private_len = private.len();
    let private_ptr = private.as_ptr();
    let cookie = xcb_xf86vidmode_mod_mode_line(c.get_raw_conn(),
        screen as u32, //1
        hdisplay as u16, //2
        hsyncstart as u16, //3
        hsyncend as u16, //4
        htotal as u16, //5
        hskew as u16, //6
        vdisplay as u16, //7
        vsyncstart as u16, //8
        vsyncend as u16, //9
        vtotal as u16, //10
        flags as u32, //11
        private_len as u32, //12
        private_ptr as *mut u8); //13
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SwitchModeChecked<'r> (c : &'r Connection,
                          screen : u16,
                          zoom : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_switch_mode_checked(c.get_raw_conn(),
        screen as u16, //1
        zoom as u16); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SwitchMode<'r> (c : &'r Connection,
                   screen : u16,
                   zoom : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_switch_mode(c.get_raw_conn(),
        screen as u16, //1
        zoom as u16); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetMonitorReply { base:  base::Reply<xcb_xf86vidmode_get_monitor_reply_t> }
fn mk_reply_xcb_xf86vidmode_get_monitor_reply_t(reply:*mut xcb_xf86vidmode_get_monitor_reply_t) -> GetMonitorReply { GetMonitorReply { base : base::mk_reply(reply) } }
pub fn GetMonitor<'r> (c : &'r Connection,
                   screen : u16) -> GetMonitorCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_monitor(c.get_raw_conn(),
        screen as u16); //1
    GetMonitorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetMonitorUnchecked<'r> (c : &'r Connection,
                            screen : u16) -> GetMonitorCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_monitor_unchecked(c.get_raw_conn(),
        screen as u16); //1
    GetMonitorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMonitorReply {
  pub fn hsync(&mut self) -> Vec<Syncrange> {
    unsafe { accessor!(Syncrange, xcb_xf86vidmode_get_monitor_hsync_length, xcb_xf86vidmode_get_monitor_hsync, (*self.base.reply)) }
  }

  pub fn vsync(&mut self) -> Vec<Syncrange> {
    unsafe { accessor!(Syncrange, xcb_xf86vidmode_get_monitor_vsync_length, xcb_xf86vidmode_get_monitor_vsync, (*self.base.reply)) }
  }

  pub fn vendor(&mut self) -> String {
    unsafe { accessor!(str, xcb_xf86vidmode_get_monitor_vendor_length, xcb_xf86vidmode_get_monitor_vendor, (*self.base.reply)) }
  }

  pub fn alignment_pad(&mut self) -> Vec<c_void> {
    unsafe { accessor!(c_void, xcb_xf86vidmode_get_monitor_alignment_pad_length, xcb_xf86vidmode_get_monitor_alignment_pad, (*self.base.reply)) }
  }

  pub fn model(&mut self) -> String {
    unsafe { accessor!(str, xcb_xf86vidmode_get_monitor_model_length, xcb_xf86vidmode_get_monitor_model, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetMonitorCookie<'s>, mk_reply_xcb_xf86vidmode_get_monitor_reply_t, GetMonitorReply, xcb_xf86vidmode_get_monitor_reply);

pub fn LockModeSwitchChecked<'r> (c : &'r Connection,
                              screen : u16,
                              lock : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_lock_mode_switch_checked(c.get_raw_conn(),
        screen as u16, //1
        lock as u16); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn LockModeSwitch<'r> (c : &'r Connection,
                       screen : u16,
                       lock : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_lock_mode_switch(c.get_raw_conn(),
        screen as u16, //1
        lock as u16); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetAllModeLinesReply { base:  base::Reply<xcb_xf86vidmode_get_all_mode_lines_reply_t> }
fn mk_reply_xcb_xf86vidmode_get_all_mode_lines_reply_t(reply:*mut xcb_xf86vidmode_get_all_mode_lines_reply_t) -> GetAllModeLinesReply { GetAllModeLinesReply { base : base::mk_reply(reply) } }
pub fn GetAllModeLines<'r> (c : &'r Connection,
                        screen : u16) -> GetAllModeLinesCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_all_mode_lines(c.get_raw_conn(),
        screen as u16); //1
    GetAllModeLinesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetAllModeLinesUnchecked<'r> (c : &'r Connection,
                                 screen : u16) -> GetAllModeLinesCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_all_mode_lines_unchecked(c.get_raw_conn(),
        screen as u16); //1
    GetAllModeLinesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetAllModeLinesReply {
  pub fn modeinfo(&mut self) -> ModeInfoIterator {
    unsafe { accessor!(ModeInfoIterator, xcb_xf86vidmode_get_all_mode_lines_modeinfo_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetAllModeLinesCookie<'s>, mk_reply_xcb_xf86vidmode_get_all_mode_lines_reply_t, GetAllModeLinesReply, xcb_xf86vidmode_get_all_mode_lines_reply);

pub fn AddModeLineChecked<'r> (c : &'r Connection,
                           screen : u32,
                           dotclock : Dotclock,
                           hdisplay : u16,
                           hsyncstart : u16,
                           hsyncend : u16,
                           htotal : u16,
                           hskew : u16,
                           vdisplay : u16,
                           vsyncstart : u16,
                           vsyncend : u16,
                           vtotal : u16,
                           flags : u32,
                           after_dotclock : Dotclock,
                           after_hdisplay : u16,
                           after_hsyncstart : u16,
                           after_hsyncend : u16,
                           after_htotal : u16,
                           after_hskew : u16,
                           after_vdisplay : u16,
                           after_vsyncstart : u16,
                           after_vsyncend : u16,
                           after_vtotal : u16,
                           after_flags : u32,
                           private : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let private_len = private.len();
    let private_ptr = private.as_ptr();
    let cookie = xcb_xf86vidmode_add_mode_line_checked(c.get_raw_conn(),
        screen as u32, //1
        dotclock as xcb_xf86vidmode_dotclock_t, //2
        hdisplay as u16, //3
        hsyncstart as u16, //4
        hsyncend as u16, //5
        htotal as u16, //6
        hskew as u16, //7
        vdisplay as u16, //8
        vsyncstart as u16, //9
        vsyncend as u16, //10
        vtotal as u16, //11
        flags as u32, //12
        private_len as u32, //13
        after_dotclock as xcb_xf86vidmode_dotclock_t, //14
        after_hdisplay as u16, //15
        after_hsyncstart as u16, //16
        after_hsyncend as u16, //17
        after_htotal as u16, //18
        after_hskew as u16, //19
        after_vdisplay as u16, //20
        after_vsyncstart as u16, //21
        after_vsyncend as u16, //22
        after_vtotal as u16, //23
        after_flags as u32, //24
        private_ptr as *mut u8); //25
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn AddModeLine<'r> (c : &'r Connection,
                    screen : u32,
                    dotclock : Dotclock,
                    hdisplay : u16,
                    hsyncstart : u16,
                    hsyncend : u16,
                    htotal : u16,
                    hskew : u16,
                    vdisplay : u16,
                    vsyncstart : u16,
                    vsyncend : u16,
                    vtotal : u16,
                    flags : u32,
                    after_dotclock : Dotclock,
                    after_hdisplay : u16,
                    after_hsyncstart : u16,
                    after_hsyncend : u16,
                    after_htotal : u16,
                    after_hskew : u16,
                    after_vdisplay : u16,
                    after_vsyncstart : u16,
                    after_vsyncend : u16,
                    after_vtotal : u16,
                    after_flags : u32,
                    private : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let private_len = private.len();
    let private_ptr = private.as_ptr();
    let cookie = xcb_xf86vidmode_add_mode_line(c.get_raw_conn(),
        screen as u32, //1
        dotclock as xcb_xf86vidmode_dotclock_t, //2
        hdisplay as u16, //3
        hsyncstart as u16, //4
        hsyncend as u16, //5
        htotal as u16, //6
        hskew as u16, //7
        vdisplay as u16, //8
        vsyncstart as u16, //9
        vsyncend as u16, //10
        vtotal as u16, //11
        flags as u32, //12
        private_len as u32, //13
        after_dotclock as xcb_xf86vidmode_dotclock_t, //14
        after_hdisplay as u16, //15
        after_hsyncstart as u16, //16
        after_hsyncend as u16, //17
        after_htotal as u16, //18
        after_hskew as u16, //19
        after_vdisplay as u16, //20
        after_vsyncstart as u16, //21
        after_vsyncend as u16, //22
        after_vtotal as u16, //23
        after_flags as u32, //24
        private_ptr as *mut u8); //25
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DeleteModeLineChecked<'r> (c : &'r Connection,
                              screen : u32,
                              dotclock : Dotclock,
                              hdisplay : u16,
                              hsyncstart : u16,
                              hsyncend : u16,
                              htotal : u16,
                              hskew : u16,
                              vdisplay : u16,
                              vsyncstart : u16,
                              vsyncend : u16,
                              vtotal : u16,
                              flags : u32,
                              private : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let private_len = private.len();
    let private_ptr = private.as_ptr();
    let cookie = xcb_xf86vidmode_delete_mode_line_checked(c.get_raw_conn(),
        screen as u32, //1
        dotclock as xcb_xf86vidmode_dotclock_t, //2
        hdisplay as u16, //3
        hsyncstart as u16, //4
        hsyncend as u16, //5
        htotal as u16, //6
        hskew as u16, //7
        vdisplay as u16, //8
        vsyncstart as u16, //9
        vsyncend as u16, //10
        vtotal as u16, //11
        flags as u32, //12
        private_len as u32, //13
        private_ptr as *mut u8); //14
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DeleteModeLine<'r> (c : &'r Connection,
                       screen : u32,
                       dotclock : Dotclock,
                       hdisplay : u16,
                       hsyncstart : u16,
                       hsyncend : u16,
                       htotal : u16,
                       hskew : u16,
                       vdisplay : u16,
                       vsyncstart : u16,
                       vsyncend : u16,
                       vtotal : u16,
                       flags : u32,
                       private : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let private_len = private.len();
    let private_ptr = private.as_ptr();
    let cookie = xcb_xf86vidmode_delete_mode_line(c.get_raw_conn(),
        screen as u32, //1
        dotclock as xcb_xf86vidmode_dotclock_t, //2
        hdisplay as u16, //3
        hsyncstart as u16, //4
        hsyncend as u16, //5
        htotal as u16, //6
        hskew as u16, //7
        vdisplay as u16, //8
        vsyncstart as u16, //9
        vsyncend as u16, //10
        vtotal as u16, //11
        flags as u32, //12
        private_len as u32, //13
        private_ptr as *mut u8); //14
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ValidateModeLine<'r> (c : &'r Connection,
                         screen : u32,
                         dotclock : Dotclock,
                         hdisplay : u16,
                         hsyncstart : u16,
                         hsyncend : u16,
                         htotal : u16,
                         hskew : u16,
                         vdisplay : u16,
                         vsyncstart : u16,
                         vsyncend : u16,
                         vtotal : u16,
                         flags : u32,
                         private : &[u8]) -> ValidateModeLineCookie<'r> {
  unsafe {
    let private_len = private.len();
    let private_ptr = private.as_ptr();
    let cookie = xcb_xf86vidmode_validate_mode_line(c.get_raw_conn(),
        screen as u32, //1
        dotclock as xcb_xf86vidmode_dotclock_t, //2
        hdisplay as u16, //3
        hsyncstart as u16, //4
        hsyncend as u16, //5
        htotal as u16, //6
        hskew as u16, //7
        vdisplay as u16, //8
        vsyncstart as u16, //9
        vsyncend as u16, //10
        vtotal as u16, //11
        flags as u32, //12
        private_len as u32, //13
        private_ptr as *mut u8); //14
    ValidateModeLineCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ValidateModeLineUnchecked<'r> (c : &'r Connection,
                                  screen : u32,
                                  dotclock : Dotclock,
                                  hdisplay : u16,
                                  hsyncstart : u16,
                                  hsyncend : u16,
                                  htotal : u16,
                                  hskew : u16,
                                  vdisplay : u16,
                                  vsyncstart : u16,
                                  vsyncend : u16,
                                  vtotal : u16,
                                  flags : u32,
                                  private : &[u8]) -> ValidateModeLineCookie<'r> {
  unsafe {
    let private_len = private.len();
    let private_ptr = private.as_ptr();
    let cookie = xcb_xf86vidmode_validate_mode_line_unchecked(c.get_raw_conn(),
        screen as u32, //1
        dotclock as xcb_xf86vidmode_dotclock_t, //2
        hdisplay as u16, //3
        hsyncstart as u16, //4
        hsyncend as u16, //5
        htotal as u16, //6
        hskew as u16, //7
        vdisplay as u16, //8
        vsyncstart as u16, //9
        vsyncend as u16, //10
        vtotal as u16, //11
        flags as u32, //12
        private_len as u32, //13
        private_ptr as *mut u8); //14
    ValidateModeLineCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ValidateModeLineReply {
  pub fn status(&mut self) -> u32 {
    unsafe { accessor!(status -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ValidateModeLineCookie<'s>, mk_reply_xcb_xf86vidmode_validate_mode_line_reply_t, ValidateModeLineReply, xcb_xf86vidmode_validate_mode_line_reply);

pub fn SwitchToModeChecked<'r> (c : &'r Connection,
                            screen : u32,
                            dotclock : Dotclock,
                            hdisplay : u16,
                            hsyncstart : u16,
                            hsyncend : u16,
                            htotal : u16,
                            hskew : u16,
                            vdisplay : u16,
                            vsyncstart : u16,
                            vsyncend : u16,
                            vtotal : u16,
                            flags : u32,
                            private : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let private_len = private.len();
    let private_ptr = private.as_ptr();
    let cookie = xcb_xf86vidmode_switch_to_mode_checked(c.get_raw_conn(),
        screen as u32, //1
        dotclock as xcb_xf86vidmode_dotclock_t, //2
        hdisplay as u16, //3
        hsyncstart as u16, //4
        hsyncend as u16, //5
        htotal as u16, //6
        hskew as u16, //7
        vdisplay as u16, //8
        vsyncstart as u16, //9
        vsyncend as u16, //10
        vtotal as u16, //11
        flags as u32, //12
        private_len as u32, //13
        private_ptr as *mut u8); //14
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SwitchToMode<'r> (c : &'r Connection,
                     screen : u32,
                     dotclock : Dotclock,
                     hdisplay : u16,
                     hsyncstart : u16,
                     hsyncend : u16,
                     htotal : u16,
                     hskew : u16,
                     vdisplay : u16,
                     vsyncstart : u16,
                     vsyncend : u16,
                     vtotal : u16,
                     flags : u32,
                     private : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let private_len = private.len();
    let private_ptr = private.as_ptr();
    let cookie = xcb_xf86vidmode_switch_to_mode(c.get_raw_conn(),
        screen as u32, //1
        dotclock as xcb_xf86vidmode_dotclock_t, //2
        hdisplay as u16, //3
        hsyncstart as u16, //4
        hsyncend as u16, //5
        htotal as u16, //6
        hskew as u16, //7
        vdisplay as u16, //8
        vsyncstart as u16, //9
        vsyncend as u16, //10
        vtotal as u16, //11
        flags as u32, //12
        private_len as u32, //13
        private_ptr as *mut u8); //14
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetViewPort<'r> (c : &'r Connection,
                    screen : u16) -> GetViewPortCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_view_port(c.get_raw_conn(),
        screen as u16); //1
    GetViewPortCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetViewPortUnchecked<'r> (c : &'r Connection,
                             screen : u16) -> GetViewPortCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_view_port_unchecked(c.get_raw_conn(),
        screen as u16); //1
    GetViewPortCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetViewPortReply {
  pub fn x(&mut self) -> u32 {
    unsafe { accessor!(x -> u32, (*self.base.reply)) }
  }

  pub fn y(&mut self) -> u32 {
    unsafe { accessor!(y -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetViewPortCookie<'s>, mk_reply_xcb_xf86vidmode_get_view_port_reply_t, GetViewPortReply, xcb_xf86vidmode_get_view_port_reply);

pub fn SetViewPortChecked<'r> (c : &'r Connection,
                           screen : u16,
                           x : u32,
                           y : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_set_view_port_checked(c.get_raw_conn(),
        screen as u16, //1
        x as u32, //2
        y as u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetViewPort<'r> (c : &'r Connection,
                    screen : u16,
                    x : u32,
                    y : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_set_view_port(c.get_raw_conn(),
        screen as u16, //1
        x as u32, //2
        y as u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetDotClocksReply { base:  base::Reply<xcb_xf86vidmode_get_dot_clocks_reply_t> }
fn mk_reply_xcb_xf86vidmode_get_dot_clocks_reply_t(reply:*mut xcb_xf86vidmode_get_dot_clocks_reply_t) -> GetDotClocksReply { GetDotClocksReply { base : base::mk_reply(reply) } }
pub fn GetDotClocks<'r> (c : &'r Connection,
                     screen : u16) -> GetDotClocksCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_dot_clocks(c.get_raw_conn(),
        screen as u16); //1
    GetDotClocksCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDotClocksUnchecked<'r> (c : &'r Connection,
                              screen : u16) -> GetDotClocksCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_dot_clocks_unchecked(c.get_raw_conn(),
        screen as u16); //1
    GetDotClocksCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDotClocksReply {
  pub fn clocks(&mut self) -> u32 {
    unsafe { accessor!(clocks -> u32, (*self.base.reply)) }
  }

  pub fn maxclocks(&mut self) -> u32 {
    unsafe { accessor!(maxclocks -> u32, (*self.base.reply)) }
  }

  pub fn clock(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xf86vidmode_get_dot_clocks_clock_length, xcb_xf86vidmode_get_dot_clocks_clock, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDotClocksCookie<'s>, mk_reply_xcb_xf86vidmode_get_dot_clocks_reply_t, GetDotClocksReply, xcb_xf86vidmode_get_dot_clocks_reply);

pub fn SetClientVersionChecked<'r> (c : &'r Connection,
                                major : u16,
                                minor : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_set_client_version_checked(c.get_raw_conn(),
        major as u16, //1
        minor as u16); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetClientVersion<'r> (c : &'r Connection,
                         major : u16,
                         minor : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_set_client_version(c.get_raw_conn(),
        major as u16, //1
        minor as u16); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetGammaChecked<'r> (c : &'r Connection,
                        screen : u16,
                        red : u32,
                        green : u32,
                        blue : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_set_gamma_checked(c.get_raw_conn(),
        screen as u16, //1
        red as u32, //2
        green as u32, //3
        blue as u32); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetGamma<'r> (c : &'r Connection,
                 screen : u16,
                 red : u32,
                 green : u32,
                 blue : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_set_gamma(c.get_raw_conn(),
        screen as u16, //1
        red as u32, //2
        green as u32, //3
        blue as u32); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetGamma<'r> (c : &'r Connection,
                 screen : u16) -> GetGammaCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_gamma(c.get_raw_conn(),
        screen as u16); //1
    GetGammaCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetGammaUnchecked<'r> (c : &'r Connection,
                          screen : u16) -> GetGammaCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_gamma_unchecked(c.get_raw_conn(),
        screen as u16); //1
    GetGammaCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetGammaReply {
  pub fn red(&mut self) -> u32 {
    unsafe { accessor!(red -> u32, (*self.base.reply)) }
  }

  pub fn green(&mut self) -> u32 {
    unsafe { accessor!(green -> u32, (*self.base.reply)) }
  }

  pub fn blue(&mut self) -> u32 {
    unsafe { accessor!(blue -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetGammaCookie<'s>, mk_reply_xcb_xf86vidmode_get_gamma_reply_t, GetGammaReply, xcb_xf86vidmode_get_gamma_reply);

pub struct GetGammaRampReply { base:  base::Reply<xcb_xf86vidmode_get_gamma_ramp_reply_t> }
fn mk_reply_xcb_xf86vidmode_get_gamma_ramp_reply_t(reply:*mut xcb_xf86vidmode_get_gamma_ramp_reply_t) -> GetGammaRampReply { GetGammaRampReply { base : base::mk_reply(reply) } }
pub fn GetGammaRamp<'r> (c : &'r Connection,
                     screen : u16,
                     size : u16) -> GetGammaRampCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_gamma_ramp(c.get_raw_conn(),
        screen as u16, //1
        size as u16); //2
    GetGammaRampCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetGammaRampUnchecked<'r> (c : &'r Connection,
                              screen : u16,
                              size : u16) -> GetGammaRampCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_gamma_ramp_unchecked(c.get_raw_conn(),
        screen as u16, //1
        size as u16); //2
    GetGammaRampCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetGammaRampReply {
  pub fn red(&mut self) -> Vec<u16> {
    unsafe { accessor!(u16, xcb_xf86vidmode_get_gamma_ramp_red_length, xcb_xf86vidmode_get_gamma_ramp_red, (*self.base.reply)) }
  }

  pub fn green(&mut self) -> Vec<u16> {
    unsafe { accessor!(u16, xcb_xf86vidmode_get_gamma_ramp_green_length, xcb_xf86vidmode_get_gamma_ramp_green, (*self.base.reply)) }
  }

  pub fn blue(&mut self) -> Vec<u16> {
    unsafe { accessor!(u16, xcb_xf86vidmode_get_gamma_ramp_blue_length, xcb_xf86vidmode_get_gamma_ramp_blue, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetGammaRampCookie<'s>, mk_reply_xcb_xf86vidmode_get_gamma_ramp_reply_t, GetGammaRampReply, xcb_xf86vidmode_get_gamma_ramp_reply);

pub fn SetGammaRampChecked<'r> (c : &'r Connection,
                            screen : u16,
                            red : &[u16],
                            green : &[u16],
                            blue : &[u16]) -> base::VoidCookie<'r> {
  unsafe {
    let red_len = red.len();
    let red_ptr = red.as_ptr();
    let green_ptr = green.as_ptr();
    let blue_ptr = blue.as_ptr();
    let cookie = xcb_xf86vidmode_set_gamma_ramp_checked(c.get_raw_conn(),
        screen as u16, //1
        red_len as u16, //2
        red_ptr as *mut u16, //3
        green_ptr as *mut u16, //4
        blue_ptr as *mut u16); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetGammaRamp<'r> (c : &'r Connection,
                     screen : u16,
                     red : &[u16],
                     green : &[u16],
                     blue : &[u16]) -> base::VoidCookie<'r> {
  unsafe {
    let red_len = red.len();
    let red_ptr = red.as_ptr();
    let green_ptr = green.as_ptr();
    let blue_ptr = blue.as_ptr();
    let cookie = xcb_xf86vidmode_set_gamma_ramp(c.get_raw_conn(),
        screen as u16, //1
        red_len as u16, //2
        red_ptr as *mut u16, //3
        green_ptr as *mut u16, //4
        blue_ptr as *mut u16); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetGammaRampSize<'r> (c : &'r Connection,
                         screen : u16) -> GetGammaRampSizeCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_gamma_ramp_size(c.get_raw_conn(),
        screen as u16); //1
    GetGammaRampSizeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetGammaRampSizeUnchecked<'r> (c : &'r Connection,
                                  screen : u16) -> GetGammaRampSizeCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_gamma_ramp_size_unchecked(c.get_raw_conn(),
        screen as u16); //1
    GetGammaRampSizeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetGammaRampSizeReply {
  pub fn size(&mut self) -> u16 {
    unsafe { accessor!(size -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetGammaRampSizeCookie<'s>, mk_reply_xcb_xf86vidmode_get_gamma_ramp_size_reply_t, GetGammaRampSizeReply, xcb_xf86vidmode_get_gamma_ramp_size_reply);

pub fn GetPermissions<'r> (c : &'r Connection,
                       screen : u16) -> GetPermissionsCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_permissions(c.get_raw_conn(),
        screen as u16); //1
    GetPermissionsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPermissionsUnchecked<'r> (c : &'r Connection,
                                screen : u16) -> GetPermissionsCookie<'r> {
  unsafe {
    let cookie = xcb_xf86vidmode_get_permissions_unchecked(c.get_raw_conn(),
        screen as u16); //1
    GetPermissionsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPermissionsReply {
  pub fn permissions(&mut self) -> u32 {
    unsafe { accessor!(permissions -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPermissionsCookie<'s>, mk_reply_xcb_xf86vidmode_get_permissions_reply_t, GetPermissionsReply, xcb_xf86vidmode_get_permissions_reply);

