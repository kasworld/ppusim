use std::{fs::File, io::BufReader};

use crate::tile;
use image::{self, GenericImageView, GrayImage, Pixel};

pub const UPPER_TILE_VEC_SIZE: usize = 256;

pub const LOWER_TILE_VEC_SIZE: usize = 256;

pub const TILE_VEC_SIZE: usize = UPPER_TILE_VEC_SIZE * LOWER_TILE_VEC_SIZE;

#[derive(Debug)]
pub struct TileVec(Vec<tile::Tile>);

impl TileVec {
    pub fn new_empty() -> Self {
        Self(vec![tile::new_empty(); TILE_VEC_SIZE])
    }
    pub fn new_random() -> Self {
        let mut rtn = Self::new_empty();
        for i in 0..TILE_VEC_SIZE {
            rtn.0[i] = tile::new_random();
        }
        rtn
    }

    pub fn get_at(&self, hi: u8, index: usize) -> tile::Tile {
        self.0[hi as usize * LOWER_TILE_VEC_SIZE + index]
    }

    // file must bmp 256 color grayscale
    // size TILE_WIDTH * LOWER_TILE_VEC_SIZE
    // size TILE_HEIGHT * UPPER_TILE_VEC_SIZE
    pub fn load_from_file(filename: String) -> Self {
        let mut rtn = Self(vec![tile::new_empty(); TILE_VEC_SIZE]);
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
        let width = tile::TILE_WIDTH as usize * LOWER_TILE_VEC_SIZE as usize;
        let height = tile::TILE_HEIGHT as usize * UPPER_TILE_VEC_SIZE as usize;
        assert_eq!(width, img.width() as usize);
        assert_eq!(height, img.width() as usize);
        for y in 0..height {
            let tly = y / tile::TILE_HEIGHT as usize;
            let tly_d = y % tile::TILE_HEIGHT as usize;
            for x in 0..width {
                let tlx = x / tile::TILE_WIDTH as usize;
                let tlx_d = x % tile::TILE_WIDTH as usize;
                let px = img.get_pixel(x as u32, y as u32).to_luma().0[0];
                rtn.0[tlx + tly * LOWER_TILE_VEC_SIZE as usize][tly_d][tlx_d] = px;
            }
        }
        rtn
    }

    pub fn save_to_file(&self, filename: String) {
        let mut img = GrayImage::new(
            (LOWER_TILE_VEC_SIZE * (tile::TILE_WIDTH as usize)) as u32,
            (UPPER_TILE_VEC_SIZE * (tile::TILE_HEIGHT as usize)) as u32,
        );
        for y in 0..UPPER_TILE_VEC_SIZE {
            for x in 0..LOWER_TILE_VEC_SIZE {
                for tx in 0..tile::TILE_WIDTH {
                    let tl = self.0[x + y * LOWER_TILE_VEC_SIZE];
                    for ty in 0..tile::TILE_HEIGHT {
                        let px = tl[ty][tx];
                        img.put_pixel(
                            (x * (tile::TILE_WIDTH as usize) + tx) as u32,
                            (y * (tile::TILE_HEIGHT as usize) + ty) as u32,
                            image::Luma([px]),
                        );
                    }
                }
            }
        }
        img.save(filename).unwrap();
    }
}
