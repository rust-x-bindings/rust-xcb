extern crate xcb;

use xcb::randr;

// per https://gitlab.freedesktop.org/xorg/app/xrandr/-/blob/master/xrandr.c#L576
fn mode_refresh(mode_info: &randr::ModeInfo) -> f64 {
    let flags = mode_info.mode_flags();
    let vtotal = {
        let mut val = mode_info.vtotal();
        if (flags & randr::MODE_FLAG_DOUBLE_SCAN) != 0 {
            val *= 2;
        }
        if (flags & randr::MODE_FLAG_INTERLACE) != 0 {
            val /= 2;
        }
        val
    };

    if vtotal != 0 && mode_info.htotal() != 0 {
        (mode_info.dot_clock() as f64) / (vtotal as f64 * mode_info.htotal() as f64)
    } else {
        0.0
    }
}

fn main() {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let window_dummy = conn.generate_id();

    xcb::create_window(
        &conn,
        0,
        window_dummy,
        screen.root(),
        0,
        0,
        1,
        1,
        0,
        0,
        0,
        &[],
    );

    conn.flush();

    let cookie = randr::get_screen_resources(&conn, window_dummy);
    let reply = cookie.get_reply().unwrap();

    let modes = reply.modes();

    for (i, mode) in modes.enumerate() {
        println!("mode {}", i + 1);
        println!("\tresolution = {}x{}", mode.width(), mode.height());
        println!("\trefresh rate = {:.1}Hz", mode_refresh(&mode));
    }
}
