use crate::{palette, tile_vec, tilemap_buffer, tilemap_vec};

#[derive(Debug)]
pub struct PPU {
    pub palette: palette::Palette,
    pub tile_def: tile_vec::TileVec,
    pub tile_map_def: tilemap_vec::TileMapVec,
    pub tile_map_buffer: tilemap_buffer::TileMapBuffer,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            palette: palette::Palette::new_empty(),
            tile_def: tile_vec::TileVec::new(),
            tile_map_def: tilemap_vec::TileMapVec::new(),
            tile_map_buffer: tilemap_buffer::TileMapBuffer::new(),
        }
    }
}
