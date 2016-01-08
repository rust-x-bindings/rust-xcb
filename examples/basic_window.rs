extern crate xcb;

use xcb::base::*;
use xcb::xproto::*;

use std::iter::{Iterator};

fn main() {
    let (conn, screen_num) = Connection::connect();

    let mut setup = conn.get_setup();

    let mut screen = setup.roots().nth(screen_num as usize).unwrap();

    let window = conn.generate_id();

    let mut values = [
        (XCB_CW_BACK_PIXEL, screen.white_pixel()),
        (XCB_CW_EVENT_MASK, XCB_EVENT_MASK_EXPOSURE | XCB_EVENT_MASK_KEY_PRESS),
    ];

    CreateWindow(&conn,
        XCB_WINDOW_CLASS_COPY_FROM_PARENT as u8,
        window,
        screen.root(),
        0, 0,
        150, 150,
        10,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        &mut values);

    MapWindow(&conn,window);

    conn.flush();

    let cookie = InternAtom(&conn,0,"_TEST_ATOM");
    let rep_res = cookie.get_reply();
    match rep_res {
        Ok(mut r) => {println!("Interned Atom {}", r.atom());}
        Err(_) => { panic!("Failed to intern atom"); }
    }

    loop {
        let event = conn.wait_for_event();
        match event {
            None => { break; }
            Some(mut event) => {
                let r = event.base.response_type();
                if r == XCB_KEY_PRESS {
                    let key_press : &mut KeyPressEvent = cast_event(&mut event);
                    println!("Key '{}' pressed", key_press.detail());
                    break;
                }
            }
        }
    }
}
