use xcb::x;

fn main() -> xcb::Result<()> {
    let text = "Hello, World!";
    /* the font name can be chosen within the output of the `xlsfonts` command */
    let fontname = "lucidasanstypewriter-bold-24";

    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    /* Create IDs for different XCB objects */
    let gc: x::Gcontext = conn.generate_id();
    let font: x::Font = conn.generate_id();
    let window: x::Window = conn.generate_id();

    conn.send_request(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid: window,
        parent: screen.root(),
        x: 0,
        y: 0,
        width: 150,
        height: 150,
        border_width: 10,
        class: x::WindowClass::InputOutput,
        visual: screen.root_visual(),
        value_list: &[
            x::Cw::BackPixel(screen.white_pixel()),
            x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS),
        ],
    });

    conn.send_request(&x::MapWindow { window });

    /* Associate the x::Font object to the font we chose */
    conn.send_request(&x::OpenFont {
        fid: font,
        name: fontname.as_bytes(),
    });

    /* The font has to be associated to the graphical context
     * If no font is specified, the default font is used */
    conn.send_request(&x::CreateGc {
        cid: gc,
        drawable: x::Drawable::Window(window),
        value_list: &[
            x::Gc::Foreground(screen.white_pixel()),
            x::Gc::Background(screen.black_pixel()),
            x::Gc::Font(font), // This line is optional: if absent the default font is used
            x::Gc::GraphicsExposures(false),
        ],
    });

    conn.flush()?;

    loop {
        let event = match conn.wait_for_event() {
            Err(xcb::Error::Connection(xcb::ConnError::Connection)) => {
                // graceful shutdown, likely "x" close button clicked in title bar
                break Ok(());
            }
            Err(err) => {
                panic!("unexpected error: {:#?}", err);
            }
            Ok(event) => event,
        };
        match event {
            xcb::Event::X(x::Event::Expose(_ev)) => {
                let drawable = x::Drawable::Window(window);

                /* We display the text */
                conn.send_request(&x::ImageText8 {
                    drawable,
                    gc,
                    x: 100,
                    y: 100,
                    string: text.as_bytes(),
                });

                /* We flush the request */
                conn.flush()?;
            }

            xcb::Event::X(x::Event::KeyPress(key_press)) => {
                println!("Key '{}' pressed", key_press.detail());
                if key_press.detail() == 0x18 {
                    // Q (on qwerty)
                    break Ok(());
                }
            }
            _ => {}
        }
    }
}
