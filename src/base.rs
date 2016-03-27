/*
 * Copyright (C) 2013 James Miller <james@aatch.net>
 * Copyright (c) 2016
 *         Remi Thebault <remi.thebault@gmail.com>
 *         Thomas Bracht Laumann Jespersen <laumann.thomas@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any
 * person obtaining a copy of this software and associated
 * documentation files (the "Software"), to deal in the
 * Software without restriction, including without
 * limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software
 * is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice
 * shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
 * ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
 * TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
 * SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
 * IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */


use xproto::*;
use ffi::base::*;
use ffi::xproto::*;
#[cfg(feature="xlib_xcb")]
use ffi::xlib_xcb::*;

#[cfg(feature="xlib_xcb")]
use x11::xlib;

use libc::{self, c_int, c_char, c_void};
use std::option::Option;

use std::mem;
use std::ptr::null;
use std::marker::PhantomData;
// std::num::Zero is unstable in rustc 1.5 => remove the Zero defined
// hereunder as soon as Zero gets stabilized (or replaced by something else)
//use std::num::Zero;
use std::cmp::Ordering;
use std::ops::{BitAnd, BitOr};
use std::ffi::CString;



pub const NONE: u32 = 0;
pub const COPY_FROM_PARENT: u32 = 0;
pub const CURRENT_TIME: u32 = 0;
pub const NO_SYMBOL: u32 = 0;


pub type Extension = xcb_extension_t;


/// `StructPtr` is a wrapper for pointer to struct owned by XCB
/// that must not be freed
/// it is instead bound to the lifetime of its parent that it borrows immutably
pub struct StructPtr<'a, T: 'a> {
    pub ptr: *mut T,
    phantom: PhantomData<&'a T>
}



/// `Event` wraps a pointer to `xcb_*_event_t`
/// this pointer will be freed when the `Event` goes out of scope
pub struct Event<T> {
   pub ptr: *mut T
}

impl<T> Event<T> {
    pub fn response_type(&self) -> u8 {
        unsafe {
            let gev : *mut xcb_generic_event_t = mem::transmute(self.ptr);
            (*gev).response_type
        }
    }
}

impl<T> Drop for Event<T> {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.ptr as *mut c_void);
        }
    }
}

/// Casts the generic event to the right event. Assumes that the given
/// event is really the correct type.
pub fn cast_event<'r, T>(event : &'r GenericEvent) -> &'r T {
    // This isn't very safe... but other options incur yet more overhead
    // that I really don't want to.
    unsafe { mem::transmute(event) }
}




/// `Error` wraps a pointer to `xcb_*_error_t`
/// this pointer will be freed when the `Error` goes out of scope
#[derive(Debug)]
pub struct Error<T> {
    pub ptr: *mut T
}

impl<T> Error<T> {
    pub fn response_type(&self) -> u8 {
        unsafe {
            let ger : *mut xcb_generic_error_t = mem::transmute(self.ptr);
            (*ger).response_type
        }
    }
    pub fn error_code(&self) -> u8 {
        unsafe {
            let ger : *mut xcb_generic_error_t = mem::transmute(self.ptr);
            (*ger).error_code
        }
    }
}

impl<T> Drop for Error<T> {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.ptr as *mut c_void);
        }
    }
}

/// Casts the generic error to the right error. Assumes that the given
/// errir is really the correct type.
pub fn cast_error<'r, T>(error : &'r GenericError) -> &'r T {
    // This isn't very safe... but other options incur yet more overhead
    // that I really don't want to.
    unsafe { mem::transmute(error) }
}




/// wraps a cookie as returned by a request function
/// instantiation of `Cookie` that are not `VoidCookie`
/// should provide a `get_reply` method to return a `Reply`
pub struct Cookie<T: Copy> {
    pub cookie: T,
    pub conn: *mut xcb_connection_t,
    pub checked: bool
}

pub type VoidCookie = Cookie<xcb_void_cookie_t>;

impl VoidCookie {
    pub fn request_check(&self) -> Result<(), GenericError> {
        unsafe {
            let c : xcb_void_cookie_t = mem::transmute(self.cookie);
            let err = xcb_request_check(self.conn, c);

            if err.is_null() {
                Ok(())
            } else {
                Err(GenericError{ ptr: err })
            }
        }
    }
}



/// Wraps a pointer to a `xcb_*_reply_t`
/// the pointer is freed when the `Reply` goes out of scope
pub struct Reply<T> {
    pub ptr: *mut T
}

impl<T> Drop for Reply<T> {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.ptr as *mut c_void);
        }
    }
}


pub type GenericEvent = Event<xcb_generic_event_t>;
pub type GenericError = Error<xcb_generic_error_t>;
pub type GenericReply = Reply<xcb_generic_reply_t>;




pub type AuthInfo = xcb_auth_info_t;
//TODO: Implement wrapper functions for constructing auth_info



#[cfg(feature="xlib_xcb")]
pub enum EventQueueOwner {
    Xcb,
    Xlib
}


#[derive(Debug)]
pub enum ConnError {
    /// xcb connection errors because of socket, pipe and other stream errors.
    Connection,
    /// xcb connection shutdown because of extension not supported
    ClosedExtNotSupported,
    /// malloc(), calloc() and realloc() error upon failure, for eg ENOMEM
    ClosedMemInsufficient,
    /// Connection closed, exceeding request length that server accepts.
    ClosedReqLenExceed,
    /// Connection closed, error during parsing display string.
    ClosedParseErr,
    /// Connection closed because the server does not have a screen
    /// matching the display.
    ClosedInvalidScreen,
    /// Connection closed because some FD passing operation failed
    ClosedFdPassingFailed,
}

pub type ConnResult<T> = Result<T, ConnError>;


/// wraps an `xcb_connection_t` object
/// will call `xcb_disconnect` when the `Connection` goes out of scope
pub struct Connection {
    c:   *mut xcb_connection_t,
    #[cfg(feature="xlib_xcb")]
    dpy: *mut xlib::Display,
}

impl Connection {
    pub fn flush(&self) -> bool {
        unsafe {
            xcb_flush(self.c) > 0
        }
    }

    pub fn get_maximum_request_length(&self) -> u32 {
        unsafe {
            xcb_get_maximum_request_length(self.c)
        }
    }

    pub fn wait_for_event(&self) -> Option<GenericEvent> {
        unsafe {
            let event = xcb_wait_for_event(self.c);
            if event.is_null() {
                None
            } else {
                Some(GenericEvent { ptr: event })
            }
        }
    }

    pub fn poll_for_event(&self) -> Option<GenericEvent> {
        unsafe {
            let event = xcb_poll_for_event(self.c);
            if event.is_null() {
                None
            } else {
                Some(GenericEvent { ptr: event })
            }
        }
    }

    pub fn poll_for_queued_event(&self) -> Option<GenericEvent> {
        unsafe {
            let event = xcb_poll_for_queued_event(self.c);
            if event.is_null() {
                None
            } else {
                Some(GenericEvent { ptr: event })
            }
        }
    }

    pub fn get_setup(&self) -> Setup {
        unsafe {

            let setup = xcb_get_setup(self.c);
            if setup.is_null() {
                panic!("NULL setup on connection")
            }
            mem::transmute(setup)
        }
    }

    pub fn has_error(&self) -> ConnResult<()> {
        unsafe {
            match xcb_connection_has_error(self.c) {
                0 => { Ok(()) },
                XCB_CONN_ERROR => { Err(ConnError::Connection) },
                XCB_CONN_CLOSED_EXT_NOTSUPPORTED =>
                        { Err(ConnError::ClosedExtNotSupported) },
                XCB_CONN_CLOSED_MEM_INSUFFICIENT =>
                        { Err(ConnError::ClosedMemInsufficient) },
                XCB_CONN_CLOSED_REQ_LEN_EXCEED =>
                        { Err(ConnError::ClosedReqLenExceed) },
                XCB_CONN_CLOSED_PARSE_ERR =>
                        { Err(ConnError::ClosedParseErr) },
                XCB_CONN_CLOSED_INVALID_SCREEN =>
                        { Err(ConnError::ClosedInvalidScreen) },
                XCB_CONN_CLOSED_FDPASSING_FAILED =>
                        { Err(ConnError::ClosedFdPassingFailed) },
                _ => {
                    panic!("unexpected error code");
                },
            }
        }
    }

    pub fn generate_id(&self) -> Window {
        unsafe {
            xcb_generate_id(self.c)
        }
    }

    pub fn get_raw_conn(&self) -> *mut xcb_connection_t {
        self.c
    }

    #[cfg(feature="xlib_xcb")]
    pub fn get_raw_dpy(&self) -> *mut xlib::Display {
        self.dpy
    }

    pub fn send_event<T>(&self,
                  propogate: bool,
                  destination: Window,
                  event_mask : u32,
                  event : Event<T>) {
        unsafe {
        xcb_send_event(self.c,
            propogate as u8, destination as xcb_window_t,
            event_mask, event.ptr as *mut c_char);
        }
    }


    pub fn prefetch_extension_data(&self, ext: &mut Extension) {
        unsafe {
            xcb_prefetch_extension_data(self.c, ext);
        }
    }


    pub fn get_extension_data(&self, ext: &mut Extension)
            -> Option<QueryExtensionData> {
        unsafe {
            let ptr = xcb_get_extension_data(self.c, ext);
            if !ptr.is_null() { Some(QueryExtensionData { ptr: ptr }) }
            else { None }
        }
    }

    #[cfg(feature="xlib_xcb")]
    pub fn set_event_queue_owner(&self, owner: EventQueueOwner) {
        debug_assert!(!self.dpy.is_null());
        unsafe {
            let owner = match owner { EventQueueOwner::Xcb => XCBOwnsEventQueue,
                EventQueueOwner::Xlib => XlibOwnsEventQueue
            };
            XSetEventQueueOwner(self.dpy, owner);
        }
    }



    #[cfg(not(feature="xlib_xcb"))]
    pub fn connect(display: Option<&str>) -> ConnResult<(Connection, i32)> {
        unsafe {
            let display = display.map(|s| CString::new(s).unwrap());
            let mut screen_num : c_int = 0;
            let cconn = xcb_connect(
                display.map_or(null(), |s| s.as_ptr()),
                &mut screen_num
            );

            // xcb doc says that a valid object is always returned
            // so we simply assert without handling this in the return
            assert!(!cconn.is_null(), "had incorrect pointer");

            let conn = Connection { c: cconn };

            conn.has_error().map(|_| {
                xcb_prefetch_maximum_request_length(cconn);
                (conn, screen_num as i32)
            })
        }
    }

    #[cfg(feature="xlib_xcb")]
    pub fn connect_with_xlib_display() -> ConnResult<(Connection, i32)> {
        unsafe {
            let dpy = xlib::XOpenDisplay(null());
            let cconn = XGetXCBConnection(dpy);
            assert!(!dpy.is_null() && !cconn.is_null(),
                "XLib could not connect to the X server");

            let conn = Connection { c: cconn, dpy: dpy };

            conn.has_error().map(|_| {
                xcb_prefetch_maximum_request_length(cconn);
                (conn, xlib::XDefaultScreen(dpy) as i32)
            })
        }
    }

    #[cfg(feature="xlib_xcb")]
    pub fn new_from_xlib_display(dpy: *mut xlib::Display) -> Connection {
        unsafe {
            assert!(!dpy.is_null(), "attempt connect with null display");
            Connection {
                c: XGetXCBConnection(dpy),
                dpy: dpy
            }
        }
    }



    #[cfg(not(feature="xlib_xcb"))]
    pub fn connect_with_auth_info(display: Option<&str>, auth_info: &AuthInfo)
    -> ConnResult<(Connection, i32)> {
        unsafe {
            let display = display.map(|s| CString::new(s).unwrap());
            let mut screen_num : c_int = 0;
            let cconn = xcb_connect_to_display_with_auth_info(
                display.map_or(null(), |s| s.as_ptr()),
                mem::transmute(auth_info),
                &mut screen_num
            );

            // xcb doc says that a valid object is always returned
            // so we simply assert without handling this in the return
            assert!(!cconn.is_null(), "had incorrect pointer");

            let conn = Connection { c: cconn };

            conn.has_error().map(|_| {
                xcb_prefetch_maximum_request_length(cconn);
                (conn, screen_num as i32)
            })
        }
    }

    #[cfg(not(feature="xlib_xcb"))]
    pub unsafe fn from_raw_conn(conn: *mut xcb_connection_t) -> Connection {
        assert!(!conn.is_null());

        Connection {
            c:  conn,
        }
    }

}

impl Drop for Connection {
    #[cfg(not(feature="xlib_xcb"))]
    fn drop(&mut self) {
        unsafe {
            xcb_disconnect(self.c);
        }
    }

    #[cfg(feature="xlib_xcb")]
    fn drop(&mut self) {
        unsafe {
            if self.dpy.is_null() {
                xcb_disconnect(self.c);
            }
            else {
                xlib::XCloseDisplay(self.dpy);
            }
        }
    }
}


// Mimics xproto::QueryExtensionReply, but without the Drop trait.
// Used for Connection::get_extension_data whose returned value
// must not be freed.
// Named QueryExtensionData to avoid name collision
pub struct QueryExtensionData {
    ptr: *const xcb_query_extension_reply_t
}

impl QueryExtensionData {
    pub fn present(&self) -> u8 {
        unsafe {
            (*self.ptr).present
        }
    }
    pub fn major_opcode(&self) -> u8 {
        unsafe {
            (*self.ptr).major_opcode
        }
    }
    pub fn first_event(&self) -> u8 {
        unsafe {
            (*self.ptr).first_event
        }
    }
    pub fn first_error(&self) -> u8 {
        unsafe {
            (*self.ptr).first_error
        }
    }
}


pub trait Zero {
    fn zero() -> Self;
}

impl Zero for u8    { fn zero() -> u8    {0} }
impl Zero for u16   { fn zero() -> u16   {0} }
impl Zero for u32   { fn zero() -> u32   {0} }
impl Zero for u64   { fn zero() -> u64   {0} }
impl Zero for usize { fn zero() -> usize {0} }
impl Zero for i8    { fn zero() -> i8    {0} }
impl Zero for i16   { fn zero() -> i16   {0} }
impl Zero for i32   { fn zero() -> i32   {0} }
impl Zero for i64   { fn zero() -> i64   {0} }
impl Zero for isize { fn zero() -> isize {0} }
impl Zero for f32   { fn zero() -> f32   {0f32} }
impl Zero for f64   { fn zero() -> f64   {0f64} }

pub fn pack_bitfield<T, L>(bf : &mut Vec<(T,L)>) -> (T, Vec<L>)
    where T: Ord + Zero + Copy + BitAnd<Output=T> + BitOr<Output=T>,
          L: Copy {
	bf.sort_by(|a,b| {
        let &(a, _) = a;
        let &(b, _) = b;
        if a < b {
            Ordering::Less
        }
        else if a > b {
            Ordering::Greater
        }
        else {
            Ordering::Equal
        }
    });

    let mut mask = T::zero();
    let mut list: Vec<L> = Vec::new();

    for el in bf.iter() {
        let &(f, v) = el;
        if mask & f > T::zero() {
            continue;
        } else {
            mask = mask|f;
            list.push(v);
        }
    }

    (mask, list)
}
