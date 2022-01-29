use crate::base::{self, WiredIn, WiredOut};
use crate::x;

fn visual_data(bits_per_rgb_value: u8) -> Vec<u8> {
    vec![
        0x50,
        0x40,
        0x30,
        0x20,               // visual_id
        3,                  // visual class
        bits_per_rgb_value, // bits_per_rgb_value
        0x50,
        0x40, // colormap_entries
        0xff,
        0x00,
        0x00,
        0x00, // red_mask
        0x00,
        0xff,
        0x00,
        0x00, // green_mask
        0x00,
        0x00,
        0xff,
        0x00, // blue_mask
        0x00,
        0x00,
        0x00,
        0x00, // pad
    ]
}

fn depth_data(visual_bprv: &[u8], extra_garbage: usize) -> Vec<u8> {
    let data: &[u8] = &[
        16,   // depth
        0x00, // pad
        0x02, 0x00, // visuals_len
        0x00, 0x00, 0x00, 0x00, // pad
    ];
    let mut vec = Vec::from(data);
    let visuals_data = visual_bprv.iter().map(|bprv| visual_data(*bprv));
    for mut vd in visuals_data {
        vec.append(&mut vd);
    }
    // append garbage
    for g in 0..extra_garbage {
        vec.push(((g ^ extra_garbage) & 0xff) as u8);
    }
    vec
}

#[test]
#[cfg(target_endian = "little")]
fn test_fixbuf_from_data() {
    let data = visual_data(24);

    let vt = unsafe { x::Visualtype::from_data(&data) };

    assert_eq!(vt.visual_id(), 0x20304050);
    assert_eq!(vt.class(), x::VisualClass::PseudoColor);
    assert_eq!(vt.bits_per_rgb_value(), 24);
    assert_eq!(vt.colormap_entries(), 0x4050);
    assert_eq!(vt.red_mask(), 0x000000ff);
    assert_eq!(vt.green_mask(), 0x0000ff00);
    assert_eq!(vt.blue_mask(), 0x00ff0000);
}

#[test]
#[cfg(target_endian = "little")]
fn test_dynbuf_from_data() {
    let mut data = depth_data(&[32, 24], 4);

    assert_eq!(data.len(), 60);
    assert_eq!(
        unsafe { <&x::Depth as WiredIn>::compute_wire_len(data.as_ptr(), ()) },
        56
    );
    {
        let depth = unsafe { x::Depth::from_data(&data[0..56]) };
        assert_eq!(depth.depth(), 16);
        let visuals = depth.visuals();
        assert_eq!(visuals.len(), 2);
        let vt = visuals[0];
        assert_eq!(vt.visual_id(), 0x20304050);
        assert_eq!(vt.class(), x::VisualClass::PseudoColor);
        assert_eq!(vt.bits_per_rgb_value(), 32);
        assert_eq!(vt.colormap_entries(), 0x4050);
        assert_eq!(vt.red_mask(), 0x000000ff);
        assert_eq!(vt.green_mask(), 0x0000ff00);
        assert_eq!(vt.blue_mask(), 0x00ff0000);
        let vt = visuals[1];
        assert_eq!(vt.visual_id(), 0x20304050);
        assert_eq!(vt.class(), x::VisualClass::PseudoColor);
        assert_eq!(vt.bits_per_rgb_value(), 24);
        assert_eq!(vt.colormap_entries(), 0x4050);
        assert_eq!(vt.red_mask(), 0x000000ff);
        assert_eq!(vt.green_mask(), 0x0000ff00);
        assert_eq!(vt.blue_mask(), 0x00ff0000);
    }

    // pop garbage
    data.pop();
    data.pop();
    data.pop();
    data.pop();

    let depth = unsafe { x::DepthBuf::from_data(data) };
    assert_eq!(depth.depth(), 16);
    let visuals = depth.visuals();
    assert_eq!(visuals.len(), 2);
    let vt = visuals[0];
    assert_eq!(vt.visual_id(), 0x20304050);
    assert_eq!(vt.class(), x::VisualClass::PseudoColor);
    assert_eq!(vt.bits_per_rgb_value(), 32);
    assert_eq!(vt.colormap_entries(), 0x4050);
    assert_eq!(vt.red_mask(), 0x000000ff);
    assert_eq!(vt.green_mask(), 0x0000ff00);
    assert_eq!(vt.blue_mask(), 0x00ff0000);
    let vt = visuals[1];
    assert_eq!(vt.visual_id(), 0x20304050);
    assert_eq!(vt.class(), x::VisualClass::PseudoColor);
    assert_eq!(vt.bits_per_rgb_value(), 24);
    assert_eq!(vt.colormap_entries(), 0x4050);
    assert_eq!(vt.red_mask(), 0x000000ff);
    assert_eq!(vt.green_mask(), 0x0000ff00);
    assert_eq!(vt.blue_mask(), 0x00ff0000);
}

#[test]
#[should_panic]
#[cfg(target_endian = "little")]
fn test_dynbuf_from_data_panic() {
    let data = depth_data(&[32, 24], 4);
    unsafe { x::DepthBuf::from_data(data) };
}

#[test]
fn test_cw_is_sorted_distinct() {
    assert!(x::Cw::is_sorted_distinct(&[
        x::Cw::BackPixel(4),
        x::Cw::BitGravity(x::Gravity::West),
        x::Cw::WinGravity(x::Gravity::West),
    ]));

    assert!(!x::Cw::is_sorted_distinct(&[
        x::Cw::BackPixel(4),
        x::Cw::WinGravity(x::Gravity::West),
        x::Cw::BitGravity(x::Gravity::West),
    ]));

    assert!(!x::Cw::is_sorted_distinct(&[
        x::Cw::BackPixel(4),
        x::Cw::BitGravity(x::Gravity::West),
        x::Cw::BitGravity(x::Gravity::West),
    ]));

    assert!(!x::Cw::is_sorted_distinct(&[
        x::Cw::BackPixel(4),
        x::Cw::BitGravity(x::Gravity::West),
        x::Cw::BitGravity(x::Gravity::Center),
    ]));
}
