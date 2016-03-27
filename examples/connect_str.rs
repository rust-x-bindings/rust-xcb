
extern crate xcb;

fn main() {
    let dpy = ":0";
    if let Some((_, _)) = xcb::Connection::connect_to_display(&dpy) {
        println!("Connected to X on display \"{}\"!", dpy);
    } else {
        println!("Could not connect to X!");
    }
}
