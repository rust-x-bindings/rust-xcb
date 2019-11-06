extern crate xcb;
use xcb::xproto::{MOD_MASK_ANY, GRAB_MODE_ASYNC, CW_EVENT_MASK, EVENT_MASK_KEY_PRESS, KEY_PRESS};

fn main() {
    let (conn, _) = xcb::Connection::connect(None).expect("Connection failed");
    let setup = conn.get_setup();

    // A display may consist of more than one screen, all screens have a root window
   for screen in setup.roots() {
        xcb::grab_key_checked(&conn, true, screen.root(), MOD_MASK_ANY as u16, 96 as u8, GRAB_MODE_ASYNC as u8, GRAB_MODE_ASYNC as u8)
                .request_check().expect("The key grab failed");
        let valuelist = [( CW_EVENT_MASK , EVENT_MASK_KEY_PRESS)];
        xcb::xproto::change_window_attributes_checked(&conn, screen.root(), &valuelist )
                .request_check().expect("Change of window attributes failed");           
   }

   &conn.flush();

    loop {
        if let Some(ev) = (&conn).wait_for_event() {
            if ev.response_type() & !0x80 == KEY_PRESS {
                println!("F12!");
                break;
            }            
        }
    }
}