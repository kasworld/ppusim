use crate::rgba::RGBA;

pub const UPPER_PALETTE_SIZE: usize = 256;

pub const LOWER_PALETTE_SIZE: usize = 256;

pub const PALETTE_SIZE: usize = UPPER_PALETTE_SIZE * LOWER_PALETTE_SIZE;

#[derive(Debug)]
pub struct Palette(Vec<RGBA>);

impl Palette {
    pub fn new_empty() -> Self {
        Self(vec![0; PALETTE_SIZE])
    }
    pub fn get_lower<'a>(&'a self, hi: u8) -> &'a [RGBA] {
        let start = hi as usize * LOWER_PALETTE_SIZE;
        &self.0[start..start + LOWER_PALETTE_SIZE]
    }
}
