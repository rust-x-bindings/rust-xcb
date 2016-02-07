
extern crate xcb;

fn main() {
    {
        let (_, screen) = xcb::Connection::connect();
        println!("connected on screen {}", screen);
    }
    println!("disconnected!");
}

