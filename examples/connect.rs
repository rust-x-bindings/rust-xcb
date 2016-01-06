
extern crate xcb;

use xcb::base::*;

fn main() {
    {
        let (_, screen) = Connection::connect();
        println!("connected on screen {}", screen);
    }
    println!("disconnected!");
}


