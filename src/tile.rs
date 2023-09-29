use std::mem::size_of;

// hold lower palette index
pub type PaletteIndex = u8;

pub const TILE_WIDTH: usize = 8;

pub const TILE_HEIGHT: usize = 8;

pub type Tile = [[PaletteIndex; TILE_WIDTH]; TILE_HEIGHT];

pub fn new_empty() -> Tile {
    [[0; TILE_WIDTH]; TILE_HEIGHT]
}

pub fn get_size_byte() -> usize {
    size_of::<Tile>()
}
