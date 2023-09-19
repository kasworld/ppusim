use crate::{palette, render_dst, tile_vec, tilemap_buffer};

// sprite or background plane
#[derive(Clone, Copy, Debug)]
pub struct TileMap {
    pub enable: bool,
    pub pos: (i16, i16),
    pub wh: (u8, u8),
    pub scale: (i8, i8),
    pub rotate: (u8, u8, u8),
    pub upper_palette_index: u8,
    pub upper_tilevec_index: u8,
    pub tilemap_buffer_index: u32,
}

impl TileMap {
    pub fn new_empty() -> Self {
        Self {
            pos: (0, 0),
            wh: (1, 1),
            scale: (1, 1),
            rotate: (0, 0, 0),
            upper_palette_index: 0,
            upper_tilevec_index: 0,
            tilemap_buffer_index: 0,
            enable: false,
        }
    }
    pub fn render(
        self,
        mut dst: &render_dst::RenderDst,
        tilemapbuffer: tilemap_buffer::TileMapBuffer,
        tilevec: tile_vec::TileVec,
        pal: palette::Palette,
    ) -> &render_dst::RenderDst {
        let lower_palette = pal.get_lower(self.upper_palette_index);
        let lower_tilevec = tilevec.get_lower(self.upper_tilevec_index);
        let tilemapbuff = tilemapbuffer.get_buffer(self.tilemap_buffer_index, self.wh.0, self.wh.1);
        dst
    }
}
