/*
 * Copyright (C) 2013 James Miller <james@aatch.net>
 * Copyright (c) 2016
 *         Remi Thebault <remi.thebault@gmail.com>
 *         Thomas Bracht Laumann Jespersen <laumann.thomas@gmail.com>
 * Copyright (c) 2025 Tolga Mizrak <tolga.mizrak@gmail.com>
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

#![allow(non_snake_case)]

use std::{
    error::Error,
    ffi::{CStr, CString},
    fmt::{Display, Formatter},
    path::Path,
    sync::PoisonError,
};

use libc::{c_char, c_int, c_uint, c_void, iovec};
#[cfg(feature = "xlib_xcb_dl")]
use x11_dl::xlib;

use crate::ffi::{
    xcb_auth_info_t, xcb_connection_t, xcb_extension_t, xcb_generic_error_t, xcb_generic_event_t,
    xcb_protocol_request_t, xcb_special_event_t, xcb_void_cookie_t,
};

#[derive(Debug, Clone)]
pub enum OpenError {
    Library(String),
    Symbol(String),
    InternalError,
}

impl Display for OpenError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        let detail: &str = match self {
            OpenError::Library(detail) => {
                f.write_str("Library")?;
                detail
            }
            OpenError::Symbol(detail) => {
                f.write_str("Symbol")?;
                detail
            }
            OpenError::InternalError => "",
        };
        if !detail.is_empty() {
            f.write_fmt(format_args!(" ({})", detail))?;
        }
        Ok(())
    }
}

impl Error for OpenError {}

impl<T> From<PoisonError<T>> for OpenError {
    fn from(_: PoisonError<T>) -> Self {
        OpenError::InternalError
    }
}

/// Mostly taken from x11-dl.
///
/// Wrapper around a dlopen handle so we can have sync, send and drop.
pub struct DynamicLibrary {
    handle: *mut c_void,
}

impl DynamicLibrary {
    pub fn open(name: &str) -> Result<DynamicLibrary, OpenError> {
        unsafe {
            let cname = match CString::new(name) {
                Ok(cname) => cname,
                Err(_) => {
                    return Err(OpenError::Library(String::from(
                        "library name contains NUL byte(s)",
                    )));
                }
            };

            let handle = libc::dlopen(cname.as_ptr(), libc::RTLD_LAZY);

            if handle.is_null() {
                let msg = libc::dlerror();

                if msg.is_null() {
                    return Err(OpenError::Library(String::new()));
                }

                let cmsg = CStr::from_ptr(msg as *const c_char);
                let detail = cmsg.to_string_lossy().into_owned();
                return Err(OpenError::Library(detail));
            }

            Ok(DynamicLibrary { handle })
        }
    }

    pub fn open_multi(names: &[&str]) -> Result<DynamicLibrary, OpenError> {
        let mut msgs = Vec::new();

        for name in names.iter() {
            match DynamicLibrary::open(name) {
                Ok(lib) => return Ok(lib),
                Err(err) => msgs.push(format!("{}", err)),
            }
        }

        let mut detail = String::new();

        for (i, msg) in msgs.iter().enumerate() {
            if i != 0 {
                detail.push_str("; ");
            }
            detail.push_str(msg.as_ref());
        }

        Err(OpenError::Library(detail))
    }

    pub fn symbol(&self, name: &str) -> Result<*mut c_void, OpenError> {
        unsafe {
            let cname = match CString::new(name) {
                Ok(cname) => cname,
                Err(_) => {
                    return Err(OpenError::Symbol(String::from(
                        "symbol name contains NUL byte(s)",
                    )));
                }
            };

            let sym = libc::dlsym(self.handle as *mut _, cname.as_ptr());

            if sym.is_null() {
                let msg = libc::dlerror();

                if msg.is_null() {
                    return Err(OpenError::Symbol(String::from(name)));
                }

                let cmsg = CStr::from_ptr(msg as *const c_char);
                let detail = format!("{} - {}", name, cmsg.to_string_lossy().into_owned());
                return Err(OpenError::Symbol(detail));
            }

            Ok(sym as *mut c_void)
        }
    }
}

impl Drop for DynamicLibrary {
    fn drop(&mut self) {
        unsafe {
            libc::dlclose(self.handle as *mut _);
        }
    }
}

unsafe impl Send for DynamicLibrary {}
unsafe impl Sync for DynamicLibrary {}

#[cfg(feature = "dl")]
mod dl_impl {
    use std::sync::{Arc, RwLock};

    use super::*;

    #[derive(Clone)]
    pub struct XcbLib {
        lib: Arc<DynamicLibrary>,

        pub xcb_flush: unsafe extern "C" fn(c: *mut xcb_connection_t) -> c_int,
        pub xcb_get_maximum_request_length: unsafe extern "C" fn(c: *mut xcb_connection_t) -> u32,
        pub xcb_prefetch_maximum_request_length:
            unsafe extern "C" fn(c: *mut xcb_connection_t) -> c_void,
        pub xcb_wait_for_event:
            unsafe extern "C" fn(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t,
        pub xcb_poll_for_event:
            unsafe extern "C" fn(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t,
        pub xcb_poll_for_queued_event:
            unsafe extern "C" fn(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t,
        pub xcb_poll_for_special_event: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            se: *mut xcb_special_event_t,
        ) -> *mut xcb_generic_event_t,
        pub xcb_wait_for_special_event: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            se: *mut xcb_special_event_t,
        ) -> *mut xcb_generic_event_t,
        pub xcb_register_for_special_xge: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            ext: *mut xcb_extension_t,
            eid: u32,
            stamp: *mut u32,
        ) -> *mut xcb_special_event_t,
        pub xcb_unregister_for_special_event:
            unsafe extern "C" fn(c: *mut xcb_connection_t, se: *mut xcb_special_event_t),
        pub xcb_request_check: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            cookie: xcb_void_cookie_t,
        ) -> *mut xcb_generic_error_t,
        pub xcb_discard_reply: unsafe extern "C" fn(c: *mut xcb_connection_t, sequence: c_uint),
        pub xcb_discard_reply64: unsafe extern "C" fn(c: *mut xcb_connection_t, sequence: u64),
        pub xcb_get_extension_data:
            unsafe extern "C" fn(c: *mut xcb_connection_t, ext: *mut xcb_extension_t) -> *const u8,
        pub xcb_prefetch_extension_data:
            unsafe extern "C" fn(c: *mut xcb_connection_t, ext: *mut xcb_extension_t),
        pub xcb_get_setup: unsafe extern "C" fn(c: *mut xcb_connection_t) -> *const u8,
        pub xcb_get_file_descriptor: unsafe extern "C" fn(c: *mut xcb_connection_t) -> c_int,
        pub xcb_connection_has_error: unsafe extern "C" fn(c: *mut xcb_connection_t) -> c_int,
        pub xcb_connect_to_fd: unsafe extern "C" fn(
            fd: c_int,
            auth_info: *mut xcb_auth_info_t,
        ) -> *mut xcb_connection_t,
        pub xcb_disconnect: unsafe extern "C" fn(c: *mut xcb_connection_t),
        pub xcb_parse_display: unsafe extern "C" fn(
            name: *const c_char,
            host: *mut *mut c_char,
            display: *mut c_int,
            screen: *mut c_int,
        ) -> c_int,
        pub xcb_connect: unsafe extern "C" fn(
            displayname: *const c_char,
            screenp: *mut c_int,
        ) -> *mut xcb_connection_t,
        pub xcb_connect_to_display_with_auth_info: unsafe extern "C" fn(
            display: *const c_char,
            auth: *mut xcb_auth_info_t,
            screen: *mut c_int,
        )
            -> *mut xcb_connection_t,
        pub xcb_generate_id: unsafe extern "C" fn(c: *mut xcb_connection_t) -> u32,
        #[cfg(feature = "libxcb_v1_14")]
        pub xcb_total_read: unsafe extern "C" fn(c: *mut xcb_connection_t) -> u64,
        #[cfg(feature = "libxcb_v1_14")]
        pub xcb_total_written: unsafe extern "C" fn(c: *mut xcb_connection_t) -> u64,

        // ext
        pub xcb_send_request: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            flags: c_int,
            vector: *mut iovec,
            request: *const xcb_protocol_request_t,
        ) -> c_uint,
        pub xcb_send_request_with_fds: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            flags: c_int,
            vector: *mut iovec,
            request: *const xcb_protocol_request_t,
            num_fds: c_uint,
            fds: *mut c_int,
        ) -> c_uint,
        pub xcb_send_request64: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            flags: c_int,
            vector: *mut iovec,
            request: *const xcb_protocol_request_t,
        ) -> u64,
        pub xcb_send_request_with_fds64: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            flags: c_int,
            vector: *mut iovec,
            request: *const xcb_protocol_request_t,
            num_fds: c_uint,
            fds: *mut c_int,
        ) -> u64,
        pub xcb_send_fd: unsafe extern "C" fn(c: *mut xcb_connection_t, fd: c_int),
        pub xcb_take_socket: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            return_socket: extern "C" fn(closure: *mut c_void),
            closure: *mut c_void,
            flags: c_int,
            sent: *mut u64,
        ) -> c_int,
        pub xcb_writev: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            vector: *mut iovec,
            count: c_int,
            requests: u64,
        ) -> c_int,
        pub xcb_wait_for_reply: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            request: c_uint,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut c_void,
        pub xcb_wait_for_reply64: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            request: u64,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut c_void,
        pub xcb_poll_for_reply: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            request: c_uint,
            reply: *mut *mut c_void,
            e: *mut *mut xcb_generic_error_t,
        ) -> c_int,
        pub xcb_poll_for_reply64: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            request: u64,
            reply: *mut *mut c_void,
            e: *mut *mut xcb_generic_error_t,
        ) -> c_int,
        pub xcb_get_reply_fds: unsafe extern "C" fn(
            c: *mut xcb_connection_t,
            reply: *mut c_void,
            replylen: usize,
        ) -> *mut c_int,
    }

    static CACHED: RwLock<Option<XcbLib>> = RwLock::new(None);

    impl XcbLib {
        pub fn open() -> Result<XcbLib, OpenError> {
            // Try to read at first, whether the library was already loaded.
            {
                let lock = CACHED.read()?;
                if let Some(lib) = lock.as_ref() {
                    return Ok(lib.clone());
                }
            }

            let mut lock = CACHED.write()?;
            // Between the read and write lock, someone might have already loaded the library.
            if let Some(lib) = lock.as_ref() {
                return Ok(lib.clone());
            }

            let lib = Arc::new(DynamicLibrary::open_multi(&["libxcb.so.1", "libxcb.so"])?);
            let xcb = unsafe {
                XcbLib {
                    xcb_flush: ::std::mem::transmute(lib.symbol("xcb_flush")?),
                    xcb_get_maximum_request_length: ::std::mem::transmute(
                        lib.symbol("xcb_get_maximum_request_length")?,
                    ),
                    xcb_prefetch_maximum_request_length: ::std::mem::transmute(
                        lib.symbol("xcb_prefetch_maximum_request_length")?,
                    ),
                    xcb_wait_for_event: ::std::mem::transmute(lib.symbol("xcb_wait_for_event")?),
                    xcb_poll_for_event: ::std::mem::transmute(lib.symbol("xcb_poll_for_event")?),
                    xcb_poll_for_queued_event: ::std::mem::transmute(
                        lib.symbol("xcb_poll_for_queued_event")?,
                    ),
                    xcb_poll_for_special_event: ::std::mem::transmute(
                        lib.symbol("xcb_poll_for_special_event")?,
                    ),
                    xcb_wait_for_special_event: ::std::mem::transmute(
                        lib.symbol("xcb_wait_for_special_event")?,
                    ),
                    xcb_register_for_special_xge: ::std::mem::transmute(
                        lib.symbol("xcb_register_for_special_xge")?,
                    ),
                    xcb_unregister_for_special_event: ::std::mem::transmute(
                        lib.symbol("xcb_unregister_for_special_event")?,
                    ),
                    xcb_request_check: ::std::mem::transmute(lib.symbol("xcb_request_check")?),
                    xcb_discard_reply: ::std::mem::transmute(lib.symbol("xcb_discard_reply")?),
                    xcb_discard_reply64: ::std::mem::transmute(lib.symbol("xcb_discard_reply64")?),
                    xcb_get_extension_data: ::std::mem::transmute(
                        lib.symbol("xcb_get_extension_data")?,
                    ),
                    xcb_prefetch_extension_data: ::std::mem::transmute(
                        lib.symbol("xcb_prefetch_extension_data")?,
                    ),
                    xcb_get_setup: ::std::mem::transmute(lib.symbol("xcb_get_setup")?),
                    xcb_get_file_descriptor: ::std::mem::transmute(
                        lib.symbol("xcb_get_file_descriptor")?,
                    ),
                    xcb_connection_has_error: ::std::mem::transmute(
                        lib.symbol("xcb_connection_has_error")?,
                    ),
                    xcb_connect_to_fd: ::std::mem::transmute(lib.symbol("xcb_connect_to_fd")?),
                    xcb_disconnect: ::std::mem::transmute(lib.symbol("xcb_disconnect")?),
                    xcb_parse_display: ::std::mem::transmute(lib.symbol("xcb_parse_display")?),
                    xcb_connect: ::std::mem::transmute(lib.symbol("xcb_connect")?),
                    xcb_connect_to_display_with_auth_info: ::std::mem::transmute(
                        lib.symbol("xcb_connect_to_display_with_auth_info")?,
                    ),
                    xcb_generate_id: ::std::mem::transmute(lib.symbol("xcb_generate_id")?),
                    #[cfg(feature = "libxcb_v1_14")]
                    xcb_total_read: ::std::mem::transmute(lib.symbol("xcb_total_read")?),
                    #[cfg(feature = "libxcb_v1_14")]
                    xcb_total_written: ::std::mem::transmute(lib.symbol("xcb_total_written")?),

                    // ext
                    xcb_send_request: ::std::mem::transmute(lib.symbol("xcb_send_request")?),
                    xcb_send_request_with_fds: ::std::mem::transmute(
                        lib.symbol("xcb_send_request_with_fds")?,
                    ),
                    xcb_send_request64: ::std::mem::transmute(lib.symbol("xcb_send_request64")?),
                    xcb_send_request_with_fds64: ::std::mem::transmute(
                        lib.symbol("xcb_send_request_with_fds64")?,
                    ),
                    xcb_send_fd: ::std::mem::transmute(lib.symbol("xcb_send_fd")?),
                    xcb_take_socket: ::std::mem::transmute(lib.symbol("xcb_take_socket")?),
                    xcb_writev: ::std::mem::transmute(lib.symbol("xcb_writev")?),
                    xcb_wait_for_reply: ::std::mem::transmute(lib.symbol("xcb_wait_for_reply")?),
                    xcb_wait_for_reply64: ::std::mem::transmute(
                        lib.symbol("xcb_wait_for_reply64")?,
                    ),
                    xcb_poll_for_reply: ::std::mem::transmute(lib.symbol("xcb_poll_for_reply")?),
                    xcb_poll_for_reply64: ::std::mem::transmute(
                        lib.symbol("xcb_poll_for_reply64")?,
                    ),
                    xcb_get_reply_fds: ::std::mem::transmute(lib.symbol("xcb_get_reply_fds")?),

                    lib: lib,
                }
            };
            *lock = Some(xcb.clone());
            Ok(xcb)
        }

        /// Unloads the current cached instance of the library.
        /// Doesn't prevent another open of the library and recreation of the cache,
        /// if they are used again after unloading.
        pub unsafe fn unload() -> Result<(), OpenError> {
            let mut lock = CACHED.write()?;
            *lock = None;
            Ok(())
        }
    }

    macro_rules! xcb_get_funcs {
        ($($name:ident),*) => {
            let lib = crate::ffi::dl::XcbLib::open()?;
            $(
                let $name = lib.$name;
            )*
        };
    }
    pub(crate) use xcb_get_funcs;

    macro_rules! xcb_get_funcs_expect {
        ($($name:ident),*) => {
            let lib = crate::ffi::dl::XcbLib::open().expect("xcb library not loaded");
            $(
                let $name = lib.$name;
            )*
        };
    }
    pub(crate) use xcb_get_funcs_expect;
}

#[cfg(feature = "dl")]
pub(crate) use dl_impl::*;

#[cfg(feature = "xlib_xcb_dl")]
mod xlib_xcb_dl_impl {
    use std::sync::Arc;
    use std::sync::RwLock;

    use super::*;
    use crate::ffi::XEventQueueOwner;

    #[derive(Clone)]
    pub struct XlibXcbLib {
        lib: Arc<DynamicLibrary>,
        pub XGetXCBConnection:
            unsafe extern "C" fn(dpy: *mut xlib::Display) -> *mut xcb_connection_t,
        pub XSetEventQueueOwner:
            unsafe extern "C" fn(dpy: *mut xlib::Display, owner: XEventQueueOwner),
    }

    static CACHED: RwLock<Option<XlibXcbLib>> = RwLock::new(None);

    impl XlibXcbLib {
        pub fn open() -> Result<XlibXcbLib, OpenError> {
            // Try to read at first, whether the library was already loaded.
            {
                let lock = CACHED.read()?;
                if let Some(lib) = lock.as_ref() {
                    return Ok(lib.clone());
                }
            }

            let mut lock = CACHED.write()?;
            // Between the read and write lock, someone might have already loaded the library.
            if let Some(lib) = lock.as_ref() {
                return Ok(lib.clone());
            }

            let lib = Arc::new(DynamicLibrary::open_multi(&[
                "libX11-xcb.so.1",
                "libX11-xcb.so",
            ])?);
            let xcb = unsafe {
                XlibXcbLib {
                    XGetXCBConnection: ::std::mem::transmute(lib.symbol("XGetXCBConnection")?),
                    XSetEventQueueOwner: ::std::mem::transmute(lib.symbol("XSetEventQueueOwner")?),
                    lib: lib,
                }
            };
            *lock = Some(xcb.clone());
            Ok(xcb)
        }

        /// Unloads the current cached instance of the library.
        /// Doesn't prevent another open of the library and recreation of the cache,
        /// if they are used again after unloading.
        pub unsafe fn unload() -> Result<(), OpenError> {
            let mut lock = CACHED.write()?;
            *lock = None;
            Ok(())
        }
    }
}

#[cfg(feature = "xlib_xcb_dl")]
pub(crate) use xlib_xcb_dl_impl::*;
