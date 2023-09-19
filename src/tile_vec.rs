// hold lower palette index
pub type PaletteIndex = u8;

pub const TILE_WIDTH: usize = 8;

pub const TILE_HEIGHT: usize = 8;

pub type Tile = [[PaletteIndex; TILE_WIDTH]; TILE_HEIGHT];

pub const UPPER_TILE_VEC_SIZE: usize = 256;

pub const LOWER_TILE_VEC_SIZE: usize = 256;

pub const TILE_VEC_SIZE: usize = UPPER_TILE_VEC_SIZE * LOWER_TILE_VEC_SIZE;

#[derive(Debug)]
pub struct TileVec(Vec<Tile>);

impl TileVec {
    pub fn new() -> Self {
        Self(vec![[[0; TILE_WIDTH]; TILE_HEIGHT]; TILE_VEC_SIZE])
    }
    pub fn get_lower<'a>(&'a self, hi: u8) -> &'a [Tile] {
        let start = hi as usize * LOWER_TILE_VEC_SIZE;
        &self.0[start..start + LOWER_TILE_VEC_SIZE]
    }
}
