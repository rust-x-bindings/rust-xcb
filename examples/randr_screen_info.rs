
extern crate xcb;

use xcb::base::*;
use xcb::xproto::*;
use xcb::randr;

fn main() {
    let (conn, screen_num) = Connection::connect();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let window_dummy = conn.generate_id();

    create_window(&conn, 0, window_dummy, screen.root(), 0, 0, 1, 1, 0, 0, 0, &[]);

    conn.flush();

    let cookie = randr::get_screen_info(&conn, window_dummy);
    let reply = cookie.get_reply().unwrap();
    let sizes = reply.sizes();

    for (i, size) in sizes.enumerate() {
        if i != 0 { println!(""); }
        println!("size of screen {}:", i+1);
        println!("   {} x {} ({}mm x {}mm)", size.width(), size.height(),
                size.mwidth(), size.mheight());
    }
}

