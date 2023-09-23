use image::RgbaImage;

use crate::{palette, tile_vec, tilemap, tilemap_buffer};

pub const TILE_MAP_VEC_SIZE: usize = 4096;

#[derive(Debug)]
pub struct TileMapVec(pub Vec<tilemap::TileMap>);

impl TileMapVec {
    pub fn new_empty() -> Self {
        Self(vec![tilemap::TileMap::new_empty(); TILE_MAP_VEC_SIZE])
    }

    pub fn render<'a>(
        &self,
        dst: &'a mut RgbaImage,
        tilemapbuffer: &'a tilemap_buffer::TileMapBuffer,
        tilevec: &'a tile_vec::TileVec,
        pal: &'a palette::Palette,
    ) -> &'a mut RgbaImage {
        let mut tilemap_list = vec![0usize;0];
        for i in 0..TILE_MAP_VEC_SIZE {
            if self.0[i].is_in_dst(dst.width() as usize, dst.height() as usize) {
                tilemap_list.push(i);
            }
        }
        println!("drawable tilemap {}", tilemap_list.len());
            
        let mut max_tilemap_num_rendered = 0;
        for y in 0..dst.height() {
            for x in 0..dst.width() {
                for tm_index in &tilemap_list{
                    let tm = self.0[*tm_index];
                    let pal_index = tm.get_at_dst_unchecked(x as usize, y as usize, tilemapbuffer, tilevec);
                    if pal_index == 0 {
                        continue;
                    }
                    let px = pal.get_at(tm.upper_palette_index, pal_index);
                    dst.put_pixel(x, y, px);
                    if max_tilemap_num_rendered < *tm_index {
                        max_tilemap_num_rendered = *tm_index;
                    }
                    break; // skip rendered pixel
                }
            }
        }
        println!("max rendered tilemap num {}", max_tilemap_num_rendered);
        dst
    }
}
