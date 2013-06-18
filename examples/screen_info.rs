extern mod xcb;

use std::iterator::{Iterator};
use xcb::base::*;

fn main() {
    let (conn, screen_num) = Connection::connect();

    let setup = conn.get_setup();

    let mut iter = setup.roots();

    let screen;
    loop {
        let n : Option<&xcb::xproto::Screen> = iter.next();
        match n {
            Some(s) => {
                if i == screen_num {
                    screen = s;
                    break;
                }
            }
            None => { fail!(~"Whut") }
        }
    }

    println("");
    println(fmt!("Informations of screen %?:", screen.root()));
    println(fmt!("  width..........: %?", screen.width_in_pixels()));
    println(fmt!("  height.........: %?", screen.height_in_pixels()));
    println(fmt!("  white pixel....: %?", screen.white_pixel()));
    println(fmt!("  black pixel....: %?", screen.black_pixel()));
}
