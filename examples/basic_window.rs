extern mod xcb;

use xcb::base::*;
use xcb::xproto::*;

use std::iterator::{Iterator};

fn main() {
    let (conn, _) = Connection::connect();
    let conn = ~conn;

    let setup = conn.get_setup();
    let mut iter = setup.roots();

    let screen;
    loop {
        let n : Option<&xcb::xproto::Screen> = iter.next();
        match n {
            Some(s) => {
                screen = s;
                break;
            }
            None => { fail!(~"Whut") }
        }
    }

    let window = conn.generate_id();

    let values = [
        (XCB_CW_BACK_PIXEL, screen.white_pixel()),
        (XCB_CW_EVENT_MASK, XCB_EVENT_MASK_EXPOSURE | XCB_EVENT_MASK_KEY_PRESS),
    ];

    CreateWindow(conn,
        XCB_WINDOW_CLASS_COPY_FROM_PARENT as u8,
        window,
        screen.root(),
        0, 0,
        150, 150,
        10,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        values);

    MapWindow(conn,window);

    conn.flush();

    let cookie = InternAtom(conn,0,"_TEST_ATOM");
    let rep_res = cookie.get_reply();
    match rep_res {
        Ok(r) => {println(fmt!("Interned Atom %?", r.atom()));}
        Err(_) => { fail!(~"Failed to intern atom"); }
    }

    loop {
        let event = conn.wait_for_event();
        match event {
            None => { break; }
            Some(event) => {
                let r = event.response_type();
                if r == XCB_KEY_PRESS {
                    let key_press : &KeyPressEvent = cast_event(&event);
                    println(fmt!("Key '%?' pressed", key_press.detail()));
                    break;
                }
            }
        }
    }
}
