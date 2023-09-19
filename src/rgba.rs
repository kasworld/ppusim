#[derive(Clone, Copy, Debug)]
pub struct RGBA(u8, u8, u8, u8);

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBA(r, g, b, a)
    }
    pub fn new_empty() -> Self {
        RGBA(0,0,0,0)
    }
}
