use image::RgbaImage;

use crate::{palette, tile_vec, tilemap, tilemap_buffer};

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
            rtn.0[i] = tilemap::TileMap::new_random2(i, offset, dst_w, dst_h);
            offset += rtn.0[i].calc_area();
        }
        println!(
            "total tile use {} / buf {} = {}",
            offset,
            tilemap_buffer::TILE_MAP_BUFFER_SIZE,
            offset as f64 / tilemap_buffer::TILE_MAP_BUFFER_SIZE as f64
        );
        rtn
    }

    pub fn new_tiledef_cover() -> Self {
        let mut rtn = Self::new_empty();
        // rtn.0[0] = tilemap::TileMap::new_tiledef_cover(0);
        for i in 0..256 {
            rtn.0[i] = tilemap::TileMap::new_tiledef_cover(i as u8);
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
        let mut max_tilemap_num_rendered = 0;
        for y in 0..dst.height() {
            for x in 0..dst.width() {
                let mut i = 0;
                for tm in &self.0 {
                    let pal_index =
                        tm.get_at_dst(x as usize, y as usize, tilemapbuffer, tilevec);
                    if pal_index == 0 {
                        i += 1;
                        continue;
                    }
                    let px = pal.get_at(tm.upper_palette_index, pal_index);
                    dst.put_pixel(x, y, px);
                    if max_tilemap_num_rendered < i {
                        max_tilemap_num_rendered = i;
                    }
                    break; // skip rendered pixel
                }
            }
        }
        println!("max rendered tilemap num {}", max_tilemap_num_rendered);
        dst
    }
}
