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
    pub fn get_lower<'a>(&'a self, hi: u8) -> &'a [tile::Tile] {
        let start = hi as usize * LOWER_TILE_VEC_SIZE;
        &self.0[start..start + LOWER_TILE_VEC_SIZE]
    }
}
