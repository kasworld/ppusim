use crate::rgba::{RGBA, self};
use image;
use rand::Rng;

pub const UPPER_PALETTE_SIZE: usize = 256;

pub const LOWER_PALETTE_SIZE: usize = 256;

pub const PALETTE_SIZE: usize = UPPER_PALETTE_SIZE * LOWER_PALETTE_SIZE;

#[derive(Debug)]
pub struct Palette(Vec<RGBA>);

impl Palette {
    pub fn new_empty() -> Self {
        Self(vec![rgba::new_zero(); PALETTE_SIZE])
    }
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let mut rtn = Self::new_empty();
        for i in 0..PALETTE_SIZE {
            rtn.0[i] = image::Rgba([
                rng.gen::<u8>(),
                rng.gen::<u8>(),
                rng.gen::<u8>(),
                rng.gen::<u8>(),
            ]);
        }
        rtn
    }

    pub fn get_at(&self, hi: u8, index: usize) -> RGBA {
        self.0[hi as usize * LOWER_PALETTE_SIZE + index]
    }
}
