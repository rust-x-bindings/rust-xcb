
extern crate xcb;

use xcb::base::*;
use xcb::xproto::*;
use xcb::randr::*;

fn main() {
    let (conn, screen_num) = Connection::connect();
    let mut screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
    let window_dummy = conn.generate_id();

    CreateWindow(&conn, 0, window_dummy, screen.root(), 0, 0, 1, 1, 0, 0, 0, &[]);

    conn.flush();

    let cookie = GetScreenInfo(&conn, window_dummy);
    let mut reply = cookie.get_reply().unwrap();
    let sizes = reply.sizes();

    for (i, mut size) in sizes.enumerate() {
        if i != 0 { println!(""); }
        println!("size of screen {}:", i+1);
        println!("   {} x {} ({}mm x {}mm)", size.width(), size.height(),
                size.mwidth(), size.mheight());
    }
}

