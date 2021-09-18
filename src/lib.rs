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

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_unsafe)]

//! Rust bindings to the XCB library.
//!
//! The X protocol C-language Binding (XCB - https://xcb.freedesktop.org/) is a
//! replacement for Xlib featuring a small footprint, latency hiding,
//! direct access to the protocol, improved threading support, and extensibility.
//!
//! The communication is established with the X server by the creation of a
//! `Connection` object.
//!
//! A client communicates with the server by sending requests. There are 2 types
//! of requests:
//!
//!   - void requests: requests that do not expect an answer (e.g. `ChangeProperty`)
//!   - non-void requests: requests that need a `Reply` (e.g. `GetProperty`)
//!
//! Void requests are normally unchecked. They are issued by the client, that
//! will normally not do anything else about it. For each such request, it exists
//! a variant with `_checked` suffix (e.g. `change_property` and `change_property_checked`)
//! that allows to receive a confirmation reply from the server via
//! `Connection::request_check`.
//!
//! Conversely, non-void requests are normally checked. When the client retrieves
//! the `Reply`, it can check for an `Error`. For each such request, it exists a variant
//! with `_unchecked` suffix (e.g. `get_property` and `get_property_unchecked`).
//! If this variant is used, the `Reply` is retrieved assuming that there was no error.
//!
//! The server can also communicate with clients by sending `Event`s.
//! The client listens to events with calls such as `Connection::wait_for_event`
//! (blocking) or `Connection::poll_for_event` (non-blocking).
//!
//! API documentation is detailed in modules `base` and `xproto`.
//!
//!  - `base`: contains `Connection` and a few utils
//!  - `xproto`: X protocol requests and events
//!
//! X protocol extensions are activated with cargo features

extern crate libc;
#[cfg(feature = "xlib_xcb")]
extern crate x11;

#[macro_use]
extern crate log;

pub mod base;

pub mod xproto {
    include!(concat!(env!("OUT_DIR"), "/xproto.rs"));
}
pub mod big_requests {
    include!(concat!(env!("OUT_DIR"), "/big_requests.rs"));
}
pub mod xc_misc {
    include!(concat!(env!("OUT_DIR"), "/xc_misc.rs"));
}

pub use base::*;
pub use xproto::*;

#[cfg(feature = "composite")]
pub mod composite {
    include!(concat!(env!("OUT_DIR"), "/composite.rs"));
}

#[cfg(feature = "damage")]
pub mod damage {
    include!(concat!(env!("OUT_DIR"), "/damage.rs"));
}

#[cfg(feature = "dpms")]
pub mod dpms {
    include!(concat!(env!("OUT_DIR"), "/dpms.rs"));
}

#[cfg(feature = "dri2")]
pub mod dri2 {
    include!(concat!(env!("OUT_DIR"), "/dri2.rs"));
}

#[cfg(feature = "dri3")]
pub mod dri3 {
    include!(concat!(env!("OUT_DIR"), "/dri3.rs"));
}

#[cfg(feature = "genericevent")]
pub mod genericevent {
    include!(concat!(env!("OUT_DIR"), "/genericevent.rs"));
}

#[cfg(feature = "glx")]
pub mod glx {
    include!(concat!(env!("OUT_DIR"), "/glx.rs"));
}

#[cfg(feature = "input")]
pub mod input {
    include!(concat!(env!("OUT_DIR"), "/input.rs"));
}

#[cfg(feature = "present")]
pub mod present {
    include!(concat!(env!("OUT_DIR"), "/present.rs"));
}

#[cfg(feature = "randr")]
pub mod randr {
    include!(concat!(env!("OUT_DIR"), "/randr.rs"));
}

#[cfg(feature = "record")]
pub mod record {
    include!(concat!(env!("OUT_DIR"), "/record.rs"));
}

#[cfg(feature = "render")]
pub mod render {
    include!(concat!(env!("OUT_DIR"), "/render.rs"));
}

#[cfg(feature = "res")]
pub mod res {
    include!(concat!(env!("OUT_DIR"), "/res.rs"));
}

#[cfg(feature = "screensaver")]
pub mod screensaver {
    include!(concat!(env!("OUT_DIR"), "/screensaver.rs"));
}

#[cfg(feature = "selinux")]
pub mod selinux {
    include!(concat!(env!("OUT_DIR"), "/selinux.rs"));
}

#[cfg(feature = "shape")]
pub mod shape {
    include!(concat!(env!("OUT_DIR"), "/shape.rs"));
}

#[cfg(feature = "shm")]
pub mod shm {
    include!(concat!(env!("OUT_DIR"), "/shm.rs"));
}

#[cfg(feature = "sync")]
pub mod sync {
    include!(concat!(env!("OUT_DIR"), "/sync.rs"));
}

#[cfg(feature = "test")]
pub mod test {
    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}

#[cfg(feature = "x_print")]
pub mod x_print {
    include!(concat!(env!("OUT_DIR"), "/x_print.rs"));
}

#[cfg(feature = "xevie")]
pub mod xevie {
    include!(concat!(env!("OUT_DIR"), "/xevie.rs"));
}

#[cfg(feature = "xf86dri")]
pub mod xf86dri {
    include!(concat!(env!("OUT_DIR"), "/xf86dri.rs"));
}

#[cfg(feature = "xf86vidmode")]
pub mod xf86vidmode {
    include!(concat!(env!("OUT_DIR"), "/xf86vidmode.rs"));
}

#[cfg(feature = "xfixes")]
pub mod xfixes {
    include!(concat!(env!("OUT_DIR"), "/xfixes.rs"));
}

#[cfg(feature = "xinerama")]
pub mod xinerama {
    include!(concat!(env!("OUT_DIR"), "/xinerama.rs"));
}

#[cfg(feature = "xkb")]
pub mod xkb {
    include!(concat!(env!("OUT_DIR"), "/xkb.rs"));
}

#[cfg(feature = "xvmc")]
pub mod xvmc {
    include!(concat!(env!("OUT_DIR"), "/xvmc.rs"));
}

#[cfg(feature = "xv")]
pub mod xv {
    include!(concat!(env!("OUT_DIR"), "/xv.rs"));
}

pub mod ffi {
    #![allow(non_camel_case_types)]
    #![allow(improper_ctypes)]

    pub mod base;
    pub mod xproto {
        include!(concat!(env!("OUT_DIR"), "/ffi/xproto.rs"));
    }
    pub mod big_requests {
        include!(concat!(env!("OUT_DIR"), "/ffi/big_requests.rs"));
    }
    pub mod xc_misc {
        include!(concat!(env!("OUT_DIR"), "/ffi/xc_misc.rs"));
    }

    pub use ffi::base::*;
    pub use ffi::xproto::*;

    #[cfg(feature = "xlib_xcb")]
    pub mod xlib_xcb;

    #[cfg(feature = "composite")]
    pub mod composite {
        include!(concat!(env!("OUT_DIR"), "/ffi/composite.rs"));
    }

    #[cfg(feature = "damage")]
    pub mod damage {
        include!(concat!(env!("OUT_DIR"), "/ffi/damage.rs"));
    }

    #[cfg(feature = "dpms")]
    pub mod dpms {
        include!(concat!(env!("OUT_DIR"), "/ffi/dpms.rs"));
    }

    #[cfg(feature = "dri2")]
    pub mod dri2 {
        include!(concat!(env!("OUT_DIR"), "/ffi/dri2.rs"));
    }

    #[cfg(feature = "dri3")]
    pub mod dri3 {
        include!(concat!(env!("OUT_DIR"), "/ffi/dri3.rs"));
    }

    #[cfg(feature = "genericevent")]
    pub mod genericevent {
        include!(concat!(env!("OUT_DIR"), "/ffi/genericevent.rs"));
    }

    #[cfg(feature = "glx")]
    pub mod glx {
        include!(concat!(env!("OUT_DIR"), "/ffi/glx.rs"));
    }

    #[cfg(feature = "input")]
    pub mod input {
        include!(concat!(env!("OUT_DIR"), "/ffi/input.rs"));
    }

    #[cfg(feature = "present")]
    pub mod present {
        include!(concat!(env!("OUT_DIR"), "/ffi/present.rs"));
    }

    #[cfg(feature = "randr")]
    pub mod randr {
        include!(concat!(env!("OUT_DIR"), "/ffi/randr.rs"));
    }

    #[cfg(feature = "record")]
    pub mod record {
        include!(concat!(env!("OUT_DIR"), "/ffi/record.rs"));
    }

    #[cfg(feature = "render")]
    pub mod render {
        include!(concat!(env!("OUT_DIR"), "/ffi/render.rs"));
    }

    #[cfg(feature = "res")]
    pub mod res {
        include!(concat!(env!("OUT_DIR"), "/ffi/res.rs"));
    }

    #[cfg(feature = "screensaver")]
    pub mod screensaver {
        include!(concat!(env!("OUT_DIR"), "/ffi/screensaver.rs"));
    }

    #[cfg(feature = "selinux")]
    pub mod selinux {
        include!(concat!(env!("OUT_DIR"), "/ffi/selinux.rs"));
    }

    #[cfg(feature = "shape")]
    pub mod shape {
        include!(concat!(env!("OUT_DIR"), "/ffi/shape.rs"));
    }

    #[cfg(feature = "shm")]
    pub mod shm {
        include!(concat!(env!("OUT_DIR"), "/ffi/shm.rs"));
    }

    #[cfg(feature = "sync")]
    pub mod sync {
        include!(concat!(env!("OUT_DIR"), "/ffi/sync.rs"));
    }

    #[cfg(feature = "test")]
    pub mod test {
        include!(concat!(env!("OUT_DIR"), "/ffi/test.rs"));
    }

    #[cfg(feature = "x_print")]
    pub mod x_print {
        include!(concat!(env!("OUT_DIR"), "/ffi/x_print.rs"));
    }

    #[cfg(feature = "xevie")]
    pub mod xevie {
        include!(concat!(env!("OUT_DIR"), "/ffi/xevie.rs"));
    }

    #[cfg(feature = "xf86dri")]
    pub mod xf86dri {
        include!(concat!(env!("OUT_DIR"), "/ffi/xf86dri.rs"));
    }

    #[cfg(feature = "xf86vidmode")]
    pub mod xf86vidmode {
        include!(concat!(env!("OUT_DIR"), "/ffi/xf86vidmode.rs"));
    }

    #[cfg(feature = "xfixes")]
    pub mod xfixes {
        include!(concat!(env!("OUT_DIR"), "/ffi/xfixes.rs"));
    }

    #[cfg(feature = "xinerama")]
    pub mod xinerama {
        include!(concat!(env!("OUT_DIR"), "/ffi/xinerama.rs"));
    }

    #[cfg(feature = "xkb")]
    pub mod xkb {
        include!(concat!(env!("OUT_DIR"), "/ffi/xkb.rs"));
    }

    #[cfg(feature = "xvmc")]
    pub mod xvmc {
        include!(concat!(env!("OUT_DIR"), "/ffi/xvmc.rs"));
    }

    #[cfg(feature = "xv")]
    pub mod xv {
        include!(concat!(env!("OUT_DIR"), "/ffi/xv.rs"));
    }
}
