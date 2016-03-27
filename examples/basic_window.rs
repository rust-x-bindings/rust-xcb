extern crate xcb;

use std::iter::{Iterator};

fn main() {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let window = conn.generate_id();

    let values = [
        (xcb::CW_BACK_PIXEL, screen.white_pixel()),
        (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
    ];

    xcb::create_window(&conn,
        xcb::COPY_FROM_PARENT as u8,
        window,
        screen.root(),
        0, 0,
        150, 150,
        10,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        &values);

    xcb::map_window(&conn, window);

    conn.flush();

    let cookie = xcb::intern_atom(&conn, false, "_TEST_ATOM");
    let rep_res = cookie.get_reply();
    match rep_res {
        Ok(r) => {println!("Interned Atom {}", r.atom());}
        Err(_) => { panic!("Failed to intern atom"); }
    }

    loop {
        let event = conn.wait_for_event();
        match event {
            None => { break; }
            Some(event) => {
                let r = event.response_type();
                if r == xcb::KEY_PRESS as u8 {
                    let key_press : &xcb::KeyPressEvent = xcb::cast_event(&event);
                    println!("Key '{}' pressed", key_press.detail());
                    break;
                }
            }
        }
    }
}
