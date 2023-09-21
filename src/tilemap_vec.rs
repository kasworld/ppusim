use image::RgbaImage;

use crate::{palette, rgba, tile_vec, tilemap, tilemap_buffer};

pub const TILE_MAP_VEC_SIZE: usize = 4096;

#[derive(Debug)]
pub struct TileMapVec(Vec<tilemap::TileMap>);

impl TileMapVec {
    pub fn new_empty() -> Self {
        Self(vec![tilemap::TileMap::new_empty(); TILE_MAP_VEC_SIZE])
    }
    pub fn new_random(dst_w: usize, dst_h: usize) -> Self {
        let mut offset: usize = 0;
        let mut rtn = Self::new_empty();
        for i in 0..TILE_MAP_VEC_SIZE {
            rtn.0[i] = tilemap::TileMap::new_random2(offset, dst_w, dst_h);
            offset += rtn.0[i].calc_area(); 
        }
        rtn
    }
    pub fn render<'a>(
        &self,
        dst: &'a mut RgbaImage,
        tilemapbuffer: &'a tilemap_buffer::TileMapBuffer,
        tilevec: &'a tile_vec::TileVec,
        pal: &'a palette::Palette,
    ) -> &'a mut RgbaImage {
        for y in 0..dst.height() {
            for x in 0..dst.width() {
                for tm in &self.0 {
                    let px =
                        tm.get_rbga_at_dst(x as usize, y as usize, tilemapbuffer, tilevec, pal);
                    dst.put_pixel(x, y, px);
                    if px != rgba::new_zero() {
                        break; // skip rendered pixel
                    }
                }
            }
        }
        dst
    }
}
