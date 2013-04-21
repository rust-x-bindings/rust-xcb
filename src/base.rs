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

use ll::base::*;

use core::cast;
use core::libc::{c_int,free};
use core::Option;
use core::Clone;

use xproto;

pub struct Connection {
    priv c : *connection
}

impl<'self> Connection {
    #[inline]
    fn flush(&self) -> bool {
        unsafe {
            xcb_flush(self.c) > 0
        }
    }

    #[inline]
    fn get_maximum_request_length(&self) -> u32 {
        unsafe {
            xcb_get_maximum_request_length(self.c)
        }
    }

    #[inline]
    fn wait_for_event(&self) -> Option<GenericEvent> {
        unsafe {
            let event = xcb_wait_for_event(self.c);
            if ptr::is_null(event) {
                None
            } else {
                Some(Event {event:event})
            }
        }
    }

    #[inline]
    fn poll_for_event(&self) -> Option<GenericEvent> {
        unsafe {
            let event = xcb_poll_for_event(self.c);
            if ptr::is_null(event) {
                None
            } else {
                Some(Event {event:event})
            }
        }
    }

    #[inline]
    fn poll_for_queued_event(&self) -> Option<GenericEvent> {
        unsafe {
            let event = xcb_poll_for_queued_event(self.c);
            if ptr::is_null(event) {
                None
            } else {
                Some(Event {event:event})
            }
        }
    }

    #[inline]
    fn get_setup(&self) -> &'self xproto::Setup {
        unsafe {
            let setup = xcb_get_setup(self.c);
            if ptr::is_null(setup) {
                fail!(~"NULL setup on connection")
            } else {
                cast::transmute(setup)
            }
        }
    }

    #[inline]
    fn has_error(&self) -> bool {
        unsafe {
            xcb_connection_has_error(self.c) > 0
        }
    }

    #[inline]
    fn generate_id<T>(&self) -> T {
        unsafe {
            cast::transmute(xcb_generate_id(self.c))
        }
    }

    #[inline]
    fn connect() -> Option<(Connection, int)> {
        let screen : c_int = 0;
        unsafe {
            let conn = xcb_connect(ptr::null(), ptr::addr_of(&screen));
            if ptr::is_null(conn) {
                None
            } else {
                xcb_prefetch_maximum_request_length(conn);
                Some((Connection {c:conn}, screen as int))
            }
        }
    }

    #[inline]
    fn connect_to_display(display:&str) -> Option<(Connection, int)> {
        let screen : c_int = 0;
        unsafe {
            let conn = do str::as_c_str(display) |s| {
                xcb_connect(s as *u8, ptr::addr_of(&screen))
            };
            if ptr::is_null(conn) {
                None
            } else {
                xcb_prefetch_maximum_request_length(conn);
                Some((Connection {c:conn}, screen as int))
            }
        }
    }

    #[inline]
    fn connect_with_auth(display:&str, auth_info: &AuthInfo) -> Option<(Connection, int)> {
        let screen : c_int = 0;
        unsafe {
            let conn = do str::as_c_str(display) |s| {
                xcb_connect_to_display_with_auth_info(s as *u8,
                    cast::transmute(ptr::addr_of(auth_info)),
                    ptr::addr_of(&screen))
            };
            if ptr::is_null(conn) {
                None
            } else {
                xcb_prefetch_maximum_request_length(conn);
                Some((Connection {c:conn}, screen as int))
            }
        }
    }

}

impl Drop for Connection {
    fn finalize(&self) {
        unsafe {
            xcb_disconnect(self.c);
        }
    }
}

impl Clone for Connection {
    fn clone(&self) -> Connection {
        Connection {c:self.c}
    }
}

pub struct Event<T> {
    priv event:*T
}

#[unsafe_destructor]
impl<T> Drop for Event<T> {
    fn finalize(&self) {
        use core::libc::c_void;
        unsafe {
            free(self.event as *c_void);
        }
    }
}

pub struct Error<T> {
    priv error:*T
}

#[unsafe_destructor]
impl<T> Drop for Error<T> {
    fn finalize(&self) {
        use core::libc::c_void;
        unsafe {
            free(self.error as *c_void);
        }
    }
}

pub type AuthInfo = auth_info;
//TODO: Implement wrapper functions for constructing auth_info

pub struct Struct<T> {
    priv struct_: T
}


pub struct Cookie<'self, T> {
    priv cookie: T,
    priv conn: &'self Connection,
    priv checked: bool
}

trait GetReply<'self, C, R> {
    fn get_reply(&self) -> Reply<R>;
}

impl<'self, T> Cookie<'self, T> {
    fn request_check(&self) -> Option<GenericError> {
        unsafe {
            // Crazy pointer dance to get the right bit
            // of the struct
            let c : *void_cookie = cast::transmute(ptr::addr_of(&self.cookie));
            let err = xcb_request_check(self.conn.c, *c);
            if ptr::is_null(err) {
                None
            } else {
                Some(Error {error:err})
            }
        }
    }
}

pub struct Reply<T> {
    priv reply:*T
}

#[unsafe_destructor]
impl<T> Drop for Reply<T> {
    fn finalize(&self) {
        use core::libc::c_void;
        unsafe {
            free(self.reply as *c_void);
        }
    }
}

pub type GenericReply = Reply<generic_reply>;
pub type GenericEvent = Event<generic_event>;
pub type GenericError = Error<generic_error>;
pub type VoidCookie<'self> = Cookie<'self, void_cookie>;
