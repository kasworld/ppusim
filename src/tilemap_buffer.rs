// hole lower tile vec index
pub type TileVecIndex = u8;

pub const TILE_MAP_BUFFER_SIZE: usize = 65536 * 11;

#[derive(Debug)]
pub struct TileMapBuffer(Vec<TileVecIndex>);

impl TileMapBuffer {
    pub fn new() -> Self {
        Self(vec![0; TILE_MAP_BUFFER_SIZE])
    }
    pub fn get_buffer<'a>(&'a self, tilemap_buffer_index: u32, w: u8, h: u8) -> &'a [TileVecIndex] {
        let start = tilemap_buffer_index as usize;
        let len = w as usize * h as usize;
        &self.0[start..start + len]
    }
}
