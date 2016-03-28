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

//! Rust bindings to the XCB library.
//! Look at documentation of modules base and xproto.

extern crate libc;
#[cfg(feature="xlib_xcb")]
extern crate x11;

#[macro_use]
extern crate log;

pub mod base;
pub mod xproto;

pub use base::*;
pub use xproto::*;

pub mod ffi
{
    #![allow(non_camel_case_types)]

    pub mod base;
    pub mod xproto;

    pub use ffi::base::*;
    pub use ffi::xproto::*;


    #[cfg(feature = "xlib_xcb")]
    pub mod xlib_xcb;


    #[cfg(feature = "bigreq")]
    pub mod bigreq;

    #[cfg(feature = "composite")]
    pub mod composite;

    #[cfg(feature = "damage")]
    pub mod damage;

    #[cfg(feature = "dpms")]
    pub mod dpms;

    #[cfg(feature = "dri2")]
    pub mod dri2;

    #[cfg(feature = "dri3")]
    pub mod dri3;

    #[cfg(feature = "ge")]
    pub mod ge;

    #[cfg(feature = "glx")]
    pub mod glx;

    #[cfg(feature = "present")]
    pub mod present;

    #[cfg(feature = "randr")]
    pub mod randr;

    #[cfg(feature = "record")]
    pub mod record;

    #[cfg(feature = "render")]
    pub mod render;

    #[cfg(feature = "res")]
    pub mod res;

    #[cfg(feature = "screensaver")]
    pub mod screensaver;

    #[cfg(feature = "shape")]
    pub mod shape;

    #[cfg(feature = "shm")]
    pub mod shm;

    #[cfg(feature = "sync")]
    pub mod sync;

    #[cfg(feature = "xc_misc")]
    pub mod xc_misc;

    #[cfg(feature = "xevie")]
    pub mod xevie;

    #[cfg(feature = "xf86dri")]
    pub mod xf86dri;

    #[cfg(feature = "xf86vidmode")]
    pub mod xf86vidmode;

    #[cfg(feature = "xfixes")]
    pub mod xfixes;

    #[cfg(feature = "xinerama")]
    pub mod xinerama;

    #[cfg(feature = "xinput")]
    pub mod xinput;

    #[cfg(feature = "xkb")]
    pub mod xkb;

    #[cfg(feature = "xprint")]
    pub mod xprint;

    #[cfg(feature = "xselinux")]
    pub mod xselinux;

    #[cfg(feature = "xtest")]
    pub mod xtest;

    #[cfg(feature = "xvmc")]
    pub mod xvmc;

    #[cfg(feature = "xv")]
    pub mod xv;
}


#[cfg(feature = "bigreq")]
pub mod bigreq;

#[cfg(feature = "composite")]
pub mod composite;

#[cfg(feature = "damage")]
pub mod damage;

#[cfg(feature = "dpms")]
pub mod dpms;

#[cfg(feature = "dri2")]
pub mod dri2;

#[cfg(feature = "dri3")]
pub mod dri3;

#[cfg(feature = "ge")]
pub mod ge;

#[cfg(feature = "glx")]
pub mod glx;

#[cfg(feature = "present")]
pub mod present;

#[cfg(feature = "randr")]
pub mod randr;

#[cfg(feature = "record")]
pub mod record;

#[cfg(feature = "render")]
pub mod render;

#[cfg(feature = "res")]
pub mod res;

#[cfg(feature = "screensaver")]
pub mod screensaver;

#[cfg(feature = "shape")]
pub mod shape;

#[cfg(feature = "shm")]
pub mod shm;

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "xc_misc")]
pub mod xc_misc;

#[cfg(feature = "xevie")]
pub mod xevie;

#[cfg(feature = "xf86dri")]
pub mod xf86dri;

#[cfg(feature = "xf86vidmode")]
pub mod xf86vidmode;

#[cfg(feature = "xfixes")]
pub mod xfixes;

#[cfg(feature = "xinerama")]
pub mod xinerama;

#[cfg(feature = "xinput")]
pub mod xinput;

#[cfg(feature = "xkb")]
pub mod xkb;

#[cfg(feature = "xprint")]
pub mod xprint;

#[cfg(feature = "xselinux")]
pub mod xselinux;

#[cfg(feature = "xtest")]
pub mod xtest;

#[cfg(feature = "xvmc")]
pub mod xvmc;

#[cfg(feature = "xv")]
pub mod xv;
