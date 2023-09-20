pub type RGBA = u32;

pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA{
    u32::from_ne_bytes([r,g,b,a])
}
