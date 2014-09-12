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

#![link(name="xcb",
       vers="0.3",
       uuid="ef466d26-0620-4f5f-a1d2-1bb9c628e101",
       url= "https://github.com/Aatch/rust-xcb")]

#![license = "MIT"]
#![crate_type="lib"]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(unsafe_destructor)]
pub mod ffi {
    pub mod xproto;
    pub mod base;

    pub mod bigreq;
    pub mod composite;
    pub mod damage;
    pub mod dpms;
    pub mod dri2;

    //pub mod ge; not sure about this one...

    pub mod glx;
    pub mod randr;
    pub mod record;
    pub mod render;
    pub mod screensaver;
    pub mod shape;
    pub mod shm;
    pub mod sync;
    pub mod xc_misc;
    pub mod xevie;
    pub mod xf86dri;
    //pub mod xf86vidmode; Same with this one...
    pub mod xfixes;
    pub mod xinerama;
    pub mod xinput;
    //pub mod xkb;
    pub mod xprint;

    #[cfg(enable_xselinux)]
    pub mod xselinux;
    pub mod xtest;
    pub mod xv;
    pub mod xvmc;
}

pub mod base;
pub mod macro;

pub mod xproto;

pub mod xinerama;

pub mod bigreq;
pub mod composite;
pub mod xfixes;
pub mod render;
pub mod shape;
/*
pub mod damage;
pub mod dpms;
pub mod dri2;
//pub mod ge; not sure about this one...

pub mod glx;
pub mod randr;
pub mod record;
pub mod screensaver;
pub mod shm;
pub mod sync;
pub mod xc_misc;
pub mod xevie;
pub mod xf86dri;
//pub mod xf86vidmode; Same with this one...
pub mod xinput;
//pub mod xkb;
pub mod xprint;

#[cfg(enable_xselinux)]
pub mod xselinux;
pub mod xtest;
pub mod xv;
pub mod xvmc;
*/
