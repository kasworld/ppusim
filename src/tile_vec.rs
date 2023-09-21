use crate::tile;

pub const UPPER_TILE_VEC_SIZE: usize = 256;

pub const LOWER_TILE_VEC_SIZE: usize = 256;

pub const TILE_VEC_SIZE: usize = UPPER_TILE_VEC_SIZE * LOWER_TILE_VEC_SIZE;

#[derive(Debug)]
pub struct TileVec(Vec<tile::Tile>);

impl TileVec {
    pub fn new_empty() -> Self {
        Self(vec![tile::new_empty(); TILE_VEC_SIZE])
    }
    pub fn new_random() -> Self {
        let mut rtn = Self::new_empty();
        for i in 0..TILE_VEC_SIZE {
            rtn.0[i] = tile::new_random();
        }
        rtn
    }
    pub fn get_at(&self, hi: u8, index: usize) -> tile::Tile {
        self.0[hi as usize * LOWER_TILE_VEC_SIZE + index]
    }
}
