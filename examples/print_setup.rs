extern crate xcb;
use xcb::{Connection};

fn main() {
    let (conn, _) = Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    println!("X Setup:");
    println!("    - status = {}", setup.status());
    println!("    - protocol major version = {}", setup.protocol_major_version());
    println!("    - protocol minor version = {}", setup.protocol_minor_version());
    println!("    - length = {}", setup.length());
    println!("    - release number = {}", setup.release_number());
    println!("    - resource Id base = {}", setup.resource_id_base());
    println!("    - motion buffer size = {}", setup.motion_buffer_size());
    println!("    - maximum request length = {}", setup.maximum_request_length());
    println!("    - image byte order = {:?}", setup.image_byte_order());
    println!("    - bitmap format bit order = {:?}", setup.bitmap_format_bit_order());
    println!("    - bitmap format scanline unit = {}", setup.bitmap_format_scanline_unit());
    println!("    - bitmap format scanline pad = {}", setup.bitmap_format_scanline_pad());
    println!("    - min keycode = {:?}", setup.min_keycode());
    println!("    - max keycode = {:?}", setup.max_keycode());
    println!("    - vendor = {}", setup.vendor());
    println!("    - pixmap formats = {:#?}", setup.pixmap_formats());
    println!("    - roots = {:#?}", setup.roots());
}
