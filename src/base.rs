/*
Copyright (C) 2013 James Miller <james@aatch.net>

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

*/

//extern crate extra;
extern crate libc;

// I don't know enough about the module system to figure out why I can't
// globally export the base module and see it here.
use ffi;
use ffi::base::*;

use libc::{c_int,c_char,free};
use std::option::Option;

use std::{num,ptr,vec,str,mem};
use std::marker::PhantomData;
// std::num::Zero is unstable in rustc 1.5 => remove impl copy
// hereunder as soon as Zero gets stabilized (or replaced by something else)
//use std::num::Zero;
use std::cmp::Ordering;
use std::ops::{BitAnd, BitOr};

use xproto;

pub struct Connection {
    c : *mut xcb_connection_t
}

impl<'s> Connection {
    #[inline]
    pub fn flush(&self) -> bool {
        unsafe {
            ffi::base::xcb_flush(self.c) > 0
        }
    }

    #[inline]
    pub fn get_maximum_request_length(&self) -> u32 {
        unsafe {
            ffi::base::xcb_get_maximum_request_length(self.c)
        }
    }

    #[inline]
    pub fn wait_for_event(&self) -> Option<GenericEvent> {
        unsafe {
            let event = ffi::base::xcb_wait_for_event(self.c);
            if event.is_null() {
                None
            } else {
                Some(GenericEvent { base: Event {event:event}})
            }
        }
    }

    #[inline]
    pub fn poll_for_event(&self) -> Option<GenericEvent> {
        unsafe {
            let event = ffi::base::xcb_poll_for_event(self.c);
            if event.is_null() {
                None
            } else {
                Some(GenericEvent { base: Event {event:event}})
            }
        }
    }

    #[inline]
    pub fn poll_for_queued_event(&self) -> Option<GenericEvent> {
        unsafe {
            let event = ffi::base::xcb_poll_for_queued_event(self.c);
            if event.is_null() {
                None
            } else {
                Some(GenericEvent { base : Event {event:event}})
            }
        }
    }

    //#[inline]
    //pub fn get_setup(&self) -> xproto::Setup {
    //    unsafe {

    //        let setup = ffi::base::xcb_get_setup(self.c);
    //        if setup.is_null() {
    //            panic!("NULL setup on connection")
    //        }
    //        mem::transmute(setup)
    //    }
    //}

    #[inline]
    pub fn has_error(&self) -> bool {
        unsafe {
            ffi::base::xcb_connection_has_error(self.c) > 0
        }
    }

    //#[inline]
    //pub fn generate_id(&self) -> xproto::Window {
    //    unsafe {
    //        ffi::base::xcb_generate_id(self.c)
    //    }
    //}

    #[inline]
    pub unsafe fn get_raw_conn(&self) -> *mut xcb_connection_t {
        self.c
    }

    //pub fn send_event<T>(&self,
    //              propogate: bool,
    //              destination: xproto::Window,
    //              event_mask : u32,
    //              event : Event<T>) {
    //    unsafe {
    //    ffi::xproto::xcb_send_event(self.c,
    //        propogate as u8, destination as ffi::xproto::xcb_window_t,
    //        event_mask, event.event as *mut c_char);
    //    }
    //}

    #[inline]
    pub fn connect() -> (Connection, i32) {
        let mut screen_num : c_int = 0;
        unsafe {
            let conn = ffi::base::xcb_connect(ptr::null_mut() as *mut u8, &mut screen_num);
            if conn.is_null() {
                panic!("Couldn't connect")
            } else {
                ffi::base::xcb_prefetch_maximum_request_length(conn);
                (Connection {c:conn}, screen_num as i32)
            }
        }
    }

    #[inline]
    pub fn connect_to_display(display:&str) -> Option<(Connection, i32)> {
        let mut screen : c_int = 0;
        unsafe {
            let conn = {
		let s = display.as_ptr();
                ffi::base::xcb_connect(s as *mut u8, &mut screen)
            };
            if conn.is_null() {
                None
            } else {
                ffi::base::xcb_prefetch_maximum_request_length(conn);
                Some((Connection {c:conn}, screen as i32))
            }
        }
    }

    #[inline]
    pub fn connect_with_auth(display:&str, auth_info: &AuthInfo) -> Option<(Connection, i32)> {
        let mut screen : c_int = 0;
        unsafe {
            let conn = {
		let s = display.as_ptr();
                ffi::base::xcb_connect_to_display_with_auth_info(s as *mut u8,
                    mem::transmute(auth_info), &mut screen)
            };
            if conn.is_null() {
                None
            } else {
                ffi::base::xcb_prefetch_maximum_request_length(conn);
                Some((Connection {c:conn}, screen as i32))
            }
        }
    }

    pub unsafe fn from_raw_conn(conn:*mut xcb_connection_t) -> Connection {
        if conn.is_null() {
            panic!("Cannot construct from null pointer");
        }

        Connection {c:conn}
    }

}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            ffi::base::xcb_disconnect(self.c);
        }
    }
}

pub struct Event<T> {
   pub event:*mut T
}


impl<T> Drop for Event<T> {
    fn drop(&mut self) {
        use libc::c_void;
        unsafe {
            free(self.event as *mut c_void);
        }
    }
}

#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct Error<T> {
    error:*mut T
}

pub fn mk_error<T>(err:*mut T) -> Error<T> {
    Error {error:err}
}

impl<T> Drop for Error<T> {
    fn drop(&mut self) {
        use libc::c_void;
        unsafe {
            free(self.error as *mut c_void);
        }
    }
}

pub type AuthInfo = xcb_auth_info_t;
//TODO: Implement wrapper functions for constructing auth_info

pub struct Struct<T> {
    pub strct: T
}

pub struct StructPtr<'a, T: 'a> {
    pub ptr: *mut T,
    phantom: PhantomData<&'a T>
}


pub struct Cookie<'s, T: Copy> {
    pub cookie: T,
    pub conn: &'s Connection,
    pub checked: bool
}

pub trait ReplyCookie<R> {
    fn get_reply(&self) -> Result<R, GenericError>;
}

impl<'s, T: Copy> Cookie<'s, T> {
    pub fn request_check(&self) -> Option<GenericError> {
        unsafe {
            let c : *mut xcb_void_cookie_t = mem::transmute(&self.cookie);
            let err = ffi::base::xcb_request_check(self.conn.c, *c);
            //let err = ffi::base::xcb_request_check(
            //    self.conn.c,
            //    void_cookie { sequence: self.cookie.sequence }
            //);
            if err.is_null() {
                None
            } else {
                Some(GenericError{base: Error {error:err}})
            }
        }
    }
}

pub struct Reply<T> {
    pub reply:*mut T
}

pub fn mk_reply<T>(reply:*mut T) -> Reply<T> {
    Reply {reply:reply}
}

impl<T> Drop for Reply<T> {
    fn drop(&mut self) {
        use libc::c_void;
        unsafe {
            free(self.reply as *mut c_void);
        }
    }
}

pub struct GenericReply { pub base : Reply<xcb_generic_reply_t>}
pub struct GenericEvent { pub base : Event<xcb_generic_event_t>}

#[derive(Debug)]
pub struct GenericError { pub base : Error<xcb_generic_error_t>}

pub struct VoidCookie<'s> { pub base : Cookie<'s, xcb_void_cookie_t> }

/**
 * Casts the generic event to the right event. Assumes that the given
 * event is really the correct type.
 */
#[inline(always)]
pub fn cast_event<'r, T>(event : &'r mut GenericEvent) -> &'r mut T {
    // This isn't very safe... but other options incur yet more overhead
    // that I really don't want to.
    unsafe { mem::transmute(event) }
}

//Accessor methods for all Events
pub trait EventUtil {
    #[inline(always)]
    fn response_type(&self) -> u8;
}

impl<T> EventUtil for Event<T> {
    fn response_type(&self) -> u8 {
        unsafe {
            let gev : *mut xcb_generic_event_t = mem::transmute(self.event);
            (*gev).response_type
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
