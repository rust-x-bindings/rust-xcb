extern crate xcb;

use std::iter::{Iterator};
use xcb::base::*;

fn main() {

    let setup;
    let screen_num;
    {
        let (conn, sn) = Connection::connect();
        setup = conn.get_setup();
        screen_num = sn;
    }

    let screen = setup.roots().nth(screen_num as usize).unwrap();

    println!("");
    println!("Informations of screen {}:", screen.root());
    println!("  width..........: {}", screen.width_in_pixels());
    println!("  height.........: {}", screen.height_in_pixels());
    println!("  white pixel....: {:x}", screen.white_pixel());
    println!("  black pixel....: {:x}", screen.black_pixel());
}
