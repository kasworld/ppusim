use std::{fs::File, io::BufReader};

use crate::tile;
use image::{self, GenericImageView, GrayImage, Pixel};

pub const UPPER_TILE_VEC_SIZE: usize = 256;

pub const LOWER_TILE_VEC_SIZE: usize = 256;

pub const TILE_VEC_SIZE: usize = UPPER_TILE_VEC_SIZE * LOWER_TILE_VEC_SIZE;

#[derive(Debug)]
pub struct TileVec(pub Vec<tile::Tile>);

impl TileVec {
    pub fn new_empty() -> Self {
        Self(vec![tile::new_empty(); TILE_VEC_SIZE])
    }

    pub fn get_at(&self, hi: u8, index: usize) -> tile::Tile {
        self.0[hi as usize * LOWER_TILE_VEC_SIZE + index]
    }

    // file must bmp 256 color grayscale
    // size TILE_WIDTH * LOWER_TILE_VEC_SIZE
    // size TILE_HEIGHT * UPPER_TILE_VEC_SIZE
    // 16 x 16 of ( 16 tile x 16 tile) image
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

        let sqrt_lower_tile_vec_size = 16; // sqrt(LOWER_TILE_VEC_SIZE)
        let mut tile_index = 0;
        for up_y in 0..sqrt_lower_tile_vec_size {
            for up_x in 0..sqrt_lower_tile_vec_size {
                for dn_y in 0..sqrt_lower_tile_vec_size {
                    for dn_x in 0..sqrt_lower_tile_vec_size {
                        for ty in 0..tile::TILE_HEIGHT {
                            for tx in 0..tile::TILE_WIDTH {
                                let x = (up_x * sqrt_lower_tile_vec_size + dn_x) * tile::TILE_WIDTH
                                    + tx;
                                let y = (up_y * sqrt_lower_tile_vec_size + dn_y)
                                    * tile::TILE_HEIGHT
                                    + ty;
                                let px = img.get_pixel(x as u32, y as u32).to_luma().0[0];
                                rtn.0[tile_index][ty][tx] = px;
                            }
                        }
                        tile_index += 1;
                    }
                }
            }
        }
        rtn
    }

    pub fn save_to_file(&self, filename: String) {
        let mut img = GrayImage::new(
            (LOWER_TILE_VEC_SIZE * (tile::TILE_WIDTH as usize)) as u32,
            (UPPER_TILE_VEC_SIZE * (tile::TILE_HEIGHT as usize)) as u32,
        );

        let sqrt_lower_tile_vec_size = 16; // sqrt(LOWER_TILE_VEC_SIZE)
        let mut tile_index = 0;
        for up_y in 0..sqrt_lower_tile_vec_size {
            for up_x in 0..sqrt_lower_tile_vec_size {
                for dn_y in 0..sqrt_lower_tile_vec_size {
                    for dn_x in 0..sqrt_lower_tile_vec_size {
                        for ty in 0..tile::TILE_HEIGHT {
                            for tx in 0..tile::TILE_WIDTH {
                                let x = (up_x * sqrt_lower_tile_vec_size + dn_x) * tile::TILE_WIDTH
                                    + tx;
                                let y = (up_y * sqrt_lower_tile_vec_size + dn_y)
                                    * tile::TILE_HEIGHT
                                    + ty;

                                let tl = self.0[tile_index];
                                let px = tl[ty][tx];
                                img.put_pixel(x as u32, y as u32, image::Luma([px]));
                            }
                        }
                        tile_index += 1;
                    }
                }
            }
        }
        img.save(filename).unwrap();
    }
}
