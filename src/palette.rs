use crate::rgba::{self, RGBA};
use crate::tile;
use image::{self, GenericImageView, RgbaImage};
use std::{fs::File, io::BufReader};

pub const UPPER_PALETTE_SIZE: usize = 256;

pub const LOWER_PALETTE_SIZE: usize = 256;

pub const PALETTE_SIZE: usize = UPPER_PALETTE_SIZE * LOWER_PALETTE_SIZE;

#[derive(Debug, Clone)]
pub struct Palette(pub Vec<RGBA>);

impl Palette {
    pub fn new_empty() -> Self {
        Self(vec![rgba::new_zero(); PALETTE_SIZE])
    }

    // file must bmp RGBA color
    // size LOWER_PALETTE_SIZE UPPER_PALETTE_SIZE
    pub fn load_from_file(filename: String) -> Self {
        let mut rtn = Self(vec![rgba::new_zero(); PALETTE_SIZE]);
        let f = match File::open(filename) {
            Ok(f) => f,
            Err(err) => {
                println!("open file {err}");
                return rtn;
            }
        };
        let img = match image::load(BufReader::new(f), image::ImageFormat::Bmp) {
            Ok(v) => v,
            Err(err) => {
                println!("load image {err}");
                return rtn;
            }
        };
        let width = LOWER_PALETTE_SIZE;
        let height = UPPER_PALETTE_SIZE;
        assert_eq!(width, img.width() as usize);
        assert_eq!(height, img.width() as usize);
        for y in 0..height {
            for x in 0..width {
                let px = img.get_pixel(x as u32, y as u32);
                rtn.0[x + y * LOWER_PALETTE_SIZE] = px;
            }
        }
        rtn
    }

    pub fn save_to_file(&self, filename: String) {
        let mut img = RgbaImage::new(LOWER_PALETTE_SIZE as u32, UPPER_PALETTE_SIZE as u32);
        for y in 0..UPPER_PALETTE_SIZE {
            for x in 0..LOWER_PALETTE_SIZE {
                img.put_pixel(x as u32, y as u32, self.0[x + y * LOWER_PALETTE_SIZE]);
            }
        }
        img.save(filename).unwrap();
    }

    pub fn get_at(&self, hi: u8, index: tile::PaletteIndex) -> RGBA {
        self.0[hi as usize * LOWER_PALETTE_SIZE + index as usize]
    }
}
