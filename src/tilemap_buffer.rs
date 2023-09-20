// hole lower tile vec index
pub type TileVecIndex = u8;

pub const TILE_MAP_BUFFER_SIZE: usize = 65536 * 11;

#[derive(Debug)]
pub struct TileMapBuffer(Vec<TileVecIndex>);

impl TileMapBuffer {
    pub fn new() -> Self {
        Self(vec![0; TILE_MAP_BUFFER_SIZE])
    }
    pub fn get_buffer<'a>(
        &'a self,
        start: usize,
        w: usize,
        h: usize,
    ) -> &'a [TileVecIndex] {
        &self.0[start..start + w * h]
    }
}
