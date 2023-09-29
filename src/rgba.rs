use std::mem::size_of;

use image;

pub type RGBA = image::Rgba<u8>;

pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
    image::Rgba([r, g, b, a])
}

pub fn new_zero() -> RGBA {
    image::Rgba([0, 0, 0, 0])
}

pub fn get_size_byte() -> usize {
    size_of::<RGBA>()
}
