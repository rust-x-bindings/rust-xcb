/*
 * Copyright (C) 2013 James Miller <james@aatch.net>
 * Copyright (c) 2016
 *         Remi Thebault <remi.thebault@gmail.com>
 *         Thomas Bracht Laumann Jespersen <laumann.thomas@gmail.com>
 *
 * Copyright (c) 2017-2021 Remi Thebault <remi.thebault@gmail.com>
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
#![allow(unused_parens)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::diverging_sub_expression)]

//! Rust bindings to the XCB library.
//!
//! The X protocol C-language Binding (XCB - <https://xcb.freedesktop.org/>) is a
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
//! Requests are passed to the server by filling a request structure e.g. `x::CreateWindow`
//! and passing it to `Connection::send_request`.
//!
//! The server can also communicate with clients by sending `Event`s.
//! The client listens to events with calls such as `Connection::wait_for_event`
//! (blocking) or `Connection::poll_for_event` (non-blocking).
//!
//! The `x` module contains definitions of the core X protocol.
//!
//! The protocol extensions are activated with cargo features

mod base;
mod error;
mod event;
mod ext;

pub use base::*;
pub use error::*;
pub use event::*;
pub use ext::*;

pub mod x {
    //! The core X protocol definitions

    pub use super::xproto::*;
}

pub mod ffi {
    #![allow(non_camel_case_types)]
    #![allow(improper_ctypes)]

    pub(crate) mod base;
    pub(crate) mod ext;

    #[cfg(feature = "xlib_xcb")]
    pub(crate) mod xlib_xcb;

    pub use base::*;
    pub use ext::*;

    #[cfg(feature = "xlib_xcb")]
    pub use xlib_xcb::*;
}

#[cfg(test)]
mod test;

mod xproto {
    #![allow(unused_variables)]
    #![allow(clippy::unit_arg)]
    #![allow(clippy::new_ret_no_self)]
    #![allow(clippy::too_many_arguments)]

    /// `COPY_FROM_PARENT` can be used for many `CreateWindow` fields
    pub const COPY_FROM_PARENT: u32 = 0;

    /// `CURRENT_TIME` can be used in most requests that take a `Timestamp`
    pub const CURRENT_TIME: Timestamp = 0;

    /// `NO_SYMBOL` fills in unused entries in `Keysym` tables
    pub const NO_SYMBOL: Keysym = 0;

    /// Trait for element in a property list
    ///
    /// In events (e.g. `GetProperty::value`), it allows to assert that the format
    /// correspond to the type cast and therefore to do the cast safely at runtime
    ///
    /// In request (e.g. `ChangeProperty::data`), it allows to infer the format value
    /// from the type of passed data
    pub trait PropEl {
        const FORMAT: u8;
    }

    impl PropEl for u8 {
        const FORMAT: u8 = 8;
    }

    impl PropEl for u16 {
        const FORMAT: u8 = 16;
    }

    impl PropEl for u32 {
        const FORMAT: u8 = 32;
    }

    impl PropEl for Atom {
        const FORMAT: u8 = 32;
    }

    include!(concat!(env!("OUT_DIR"), "/xproto.rs"));
}
pub mod bigreq {
    include!(concat!(env!("OUT_DIR"), "/bigreq.rs"));
}
pub mod xc_misc {
    include!(concat!(env!("OUT_DIR"), "/xc_misc.rs"));
}

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
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/dri2.rs"));
}

#[cfg(feature = "dri3")]
pub mod dri3 {
    include!(concat!(env!("OUT_DIR"), "/dri3.rs"));
}

#[cfg(feature = "ge")]
pub mod ge {
    include!(concat!(env!("OUT_DIR"), "/ge.rs"));
}

#[cfg(feature = "glx")]
pub mod glx {
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/glx.rs"));
}

#[cfg(feature = "xinput")]
pub mod xinput {
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(clippy::unit_arg)]
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/xinput.rs"));
}

#[cfg(feature = "present")]
pub mod present {
    #![allow(clippy::unit_arg)]
    include!(concat!(env!("OUT_DIR"), "/present.rs"));
}

#[cfg(feature = "randr")]
pub mod randr {
    #![allow(clippy::unit_arg)]
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/randr.rs"));
}

#[cfg(feature = "record")]
pub mod record {
    #![allow(clippy::unit_arg)]
    include!(concat!(env!("OUT_DIR"), "/record.rs"));
}

#[cfg(feature = "render")]
pub mod render {
    #![allow(unused_variables)]
    #![allow(clippy::unit_arg)]
    include!(concat!(env!("OUT_DIR"), "/render.rs"));
}

#[cfg(feature = "res")]
pub mod res {
    #![allow(unused_variables)]
    #![allow(clippy::unit_arg)]
    include!(concat!(env!("OUT_DIR"), "/res.rs"));
}

#[cfg(feature = "screensaver")]
pub mod screensaver {
    #![allow(unused_variables)]
    include!(concat!(env!("OUT_DIR"), "/screensaver.rs"));
}

#[cfg(feature = "xselinux")]
pub mod xselinux {
    #![allow(clippy::unit_arg)]
    include!(concat!(env!("OUT_DIR"), "/xselinux.rs"));
}

#[cfg(feature = "shape")]
pub mod shape {
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/shape.rs"));
}

#[cfg(feature = "shm")]
pub mod shm {
    include!(concat!(env!("OUT_DIR"), "/shm.rs"));
}

#[cfg(feature = "sync")]
pub mod sync {
    #![allow(unused_variables)]
    #![allow(clippy::unit_arg)]
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/sync.rs"));
}

#[cfg(feature = "xtest")]
pub mod xtest {
    include!(concat!(env!("OUT_DIR"), "/xtest.rs"));
}

#[cfg(feature = "xprint")]
pub mod xprint {
    #![allow(clippy::unit_arg)]
    include!(concat!(env!("OUT_DIR"), "/xprint.rs"));
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
    #![allow(unused_variables)]
    #![allow(clippy::unit_arg)]
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/xkb.rs"));
}

#[cfg(feature = "xvmc")]
pub mod xvmc {
    include!(concat!(env!("OUT_DIR"), "/xvmc.rs"));
}

#[cfg(feature = "xv")]
pub mod xv {
    #![allow(clippy::unit_arg)]
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/xv.rs"));
}
