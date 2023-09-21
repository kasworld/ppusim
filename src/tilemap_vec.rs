use crate::{palette, render_dst, tile_vec, tilemap, tilemap_buffer};

pub const TILE_MAP_VEC_SIZE: usize = 4096;

#[derive(Debug)]
pub struct TileMapVec(Vec<tilemap::TileMap>);

impl TileMapVec {
    pub fn new_empty() -> Self {
        Self(vec![tilemap::TileMap::new_empty(); TILE_MAP_VEC_SIZE])
    }
    pub fn new_random(dst_w: usize, dst_h: usize) -> Self {
        let mut rtn = Self::new_empty();
        for i in 0..TILE_MAP_VEC_SIZE {
            rtn.0[i] = tilemap::TileMap::new_random(dst_w, dst_h);
        }
        rtn
    }
    pub fn calc_render_count(self, dstw: usize, dsth: usize) -> usize {
        let mut sum = 0;
        for tm in &self.0 {
            sum += tm.calc_render_count(dstw, dsth);
        }
        sum
    }
    pub fn render<'a>(
        &self,
        mut dst: &'a mut render_dst::RenderDst,
        tilemapbuffer: &'a tilemap_buffer::TileMapBuffer,
        tilevec: &'a tile_vec::TileVec,
        pal: &'a palette::Palette,
    ) -> &'a mut render_dst::RenderDst {
        for tm in &self.0 {
            dst = tm.render(dst, tilemapbuffer, tilevec, pal)
        }
        dst
    }

    pub fn render2<'a>(
        &self,
        dst: &'a mut render_dst::RenderDst,
        tilemapbuffer: &'a tilemap_buffer::TileMapBuffer,
        tilevec: &'a tile_vec::TileVec,
        pal: &'a palette::Palette,
    ) -> &'a mut render_dst::RenderDst {
        for y in 0..dst.h {
            for x in 0..dst.w {
                let buf_pos = y * dst.w + x;
                if dst.buffer[buf_pos] != 0 {
                    continue; // skip rendered pixel
                }
                for tm in &self.0 {
                    dst.buffer[buf_pos] = tm.get_rbga_at_dst(x, y, tilemapbuffer, tilevec, pal);
                    if dst.buffer[buf_pos] != 0 {
                        break;
                    }
                }
            }
        }
        dst
    }
}
