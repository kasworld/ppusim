use crate::{palette, render_dst, tile_vec, tilemap, tilemap_buffer};

pub const TILE_MAP_VEC_SIZE: usize = 4096;

#[derive(Debug)]
pub struct TileMapVec(Vec<tilemap::TileMap>);

impl TileMapVec {
    pub fn new_empty() -> Self {
        Self(vec![tilemap::TileMap::new_empty(); TILE_MAP_VEC_SIZE])
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
}
