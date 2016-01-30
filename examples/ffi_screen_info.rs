
extern crate xcb;
extern crate libc;

use xcb::ffi::base::*;
use xcb::ffi::xproto::*;
use std::ptr;
use libc::{c_int, c_char};

fn main() {
    unsafe {
        let mut screen_num: c_int = 0;
        let c = xcb_connect(ptr::null_mut() as *mut c_char,
                &mut screen_num as *mut c_int);
        if c.is_null() { panic!(); }

        let setup = xcb_get_setup(c);
        let mut iter = xcb_setup_roots_iterator(setup);
        for _ in 0..screen_num {
            xcb_screen_next(&mut iter as *mut xcb_screen_iterator_t);
        }
        let screen = &*iter.data;
        println!("");
        println!("Informations of screen {}:", screen.root);
        println!("  width..........: {}", screen.width_in_pixels);
        println!("  height.........: {}", screen.height_in_pixels);
        println!("  white pixel....: {:x}", screen.white_pixel);
        println!("  black pixel....: {:x}", screen.black_pixel);

        xcb_disconnect(c);
    }
}

