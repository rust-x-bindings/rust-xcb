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
use std::num::*;

use xproto;

pub struct Connection {
    c : *mut connection
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
                Some(Event {event:event})
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
                Some(Event {event:event})
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
                Some(Event {event:event})
            }
        }
    }

    #[inline]
    pub fn get_setup(&self) -> &'s xproto::Setup {
        unsafe {
            let setup = ffi::base::xcb_get_setup(self.c);
            if setup.is_null() {
                fail!(box "NULL setup on connection")
            } else {
                mem::transmute(setup)
            }
        }
    }

    #[inline]
    pub fn has_error(&self) -> bool {
        unsafe {
            ffi::base::xcb_connection_has_error(self.c) > 0
        }
    }

    #[inline]
    pub fn generate_id<T>(&self) -> T {
        unsafe {
            mem::transmute(ffi::base::xcb_generate_id(self.c))
        }
    }

    #[inline]
    pub unsafe fn get_raw_conn(&self) -> *mut connection {
        self.c
    }

    pub fn send_event<T>(&self,
                  propogate: bool,
                  destination: xproto::Window,
                  event_mask : u32,
                  event : Event<T>) {
        unsafe {
        ffi::xproto::xcb_send_event(self.c,
            propogate as u8, destination as ffi::xproto::window,
            event_mask, event.event as *mut c_char);
        }
    }

    #[inline]
    pub fn connect() -> (Connection, int) {
        let screen : c_int = 0;
        unsafe {
            let conn = ffi::base::xcb_connect(ptr::mut_null(), &mut screen);
            if conn.is_null() {
                fail!(box "Couldn't connect")
            } else {
                ffi::base::xcb_prefetch_maximum_request_length(conn);
                (Connection {c:conn}, screen as int)
            }
        }
    }

    #[inline]
    pub fn connect_to_display(display:&str) -> Option<(Connection, int)> {
        let screen : c_int = 0;
        unsafe {
            let conn = {
		let s = display.as_ptr();
                ffi::base::xcb_connect(s as *mut u8, &mut screen)
            };
            if conn.is_null() {
                None
            } else {
                ffi::base::xcb_prefetch_maximum_request_length(conn);
                Some((Connection {c:conn}, screen as int))
            }
        }
    }

    #[inline]
    pub fn connect_with_auth(display:&str, auth_info: &AuthInfo) -> Option<(Connection, int)> {
        let screen : c_int = 0;
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
                Some((Connection {c:conn}, screen as int))
            }
        }
    }

    pub unsafe fn from_raw_conn(conn:*mut connection) -> Connection {
        if conn.is_null() {
            fail!("Cannot construct from null pointer");
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

#[unsafe_destructor]
impl<T> Drop for Event<T> {
    fn drop(&mut self) {
        use libc::c_void;
        unsafe {
            free(self.event as *mut c_void);
        }
    }
}

pub struct Error<T> {
    error:*mut T
}

pub fn mk_error<T>(err:*mut T) -> Error<T> {
    Error {error:err}
}

#[unsafe_destructor]
impl<T> Drop for Error<T> {
    fn drop(&mut self) {
        use libc::c_void;
        unsafe {
            free(self.error as *mut c_void);
        }
    }
}

pub type AuthInfo = auth_info;
//TODO: Implement wrapper functions for constructing auth_info

pub struct Struct<T> {
    pub strct: T
}

pub struct Cookie<'s, T> {
    pub cookie: T,
    pub conn: &'s Connection,
    pub checked: bool
}

pub trait ReplyCookie<R> {
    fn get_reply(&self) -> Result<Reply<R>, GenericError>;
}

impl<'s, T> Cookie<'s, T> {
    pub fn request_check(&self) -> Option<GenericError> {
        unsafe {
            // Crazy pointer dance to get the right bit
            // of the struct
            let c : *mut void_cookie = mem::transmute(&self.cookie);
            let err = ffi::base::xcb_request_check(self.conn.c, *c);
            if err.is_null() {
                None
            } else {
                Some(Error {error:err})
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

#[unsafe_destructor]
impl<T> Drop for Reply<T> {
    fn drop(&mut self) {
        use libc::c_void;
        unsafe {
            free(self.reply as *mut c_void);
        }
    }
}

pub type GenericReply = Reply<generic_reply>;
pub type GenericEvent = Event<generic_event>;
pub type GenericError = Error<generic_error>;
pub type VoidCookie<'s> = Cookie<'s, void_cookie>;

/**
 * Casts the generic event to the right event. Assumes that the given
 * event is really the correct type.
 */
#[inline(always)]
pub fn mem_event<'r, T>(event : &'r GenericEvent) -> &'r T {
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
            let gev : *mut generic_event = mem::transmute(self.event);
            (*gev).response_type
        }
    }
}

pub fn pack_bitfield<T:Ord+Zero+NumCast+Copy,L:Copy>(bf : &[(T,L)]) -> (T, Vec<L>) {
    bf.sort_by(|a,b| {
        let &(a, _) = a;
        let &(b, _) = b;
        if a < b { Less } else if a > b { Greater } else { Equal }       
        });
    
    let mut mask = 0u;
    let mut list : Vec<L> = Vec::new();

    for el in bf.iter() {
        let &(f, v) = el;
        let fld= num::cast(f).unwrap();
        if (mask & fld) > 0 {
            continue;
        } else {
            mask |= fld;
            list.push(v);
        }
    }

    (num::cast(mask).unwrap(), list)
}
