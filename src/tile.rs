use rand::Rng;

// hold lower palette index
pub type PaletteIndex = u8;

pub const TILE_WIDTH: usize = 8;

pub const TILE_HEIGHT: usize = 8;

pub type Tile = [[PaletteIndex; TILE_WIDTH]; TILE_HEIGHT];

pub fn new_empty() -> Tile {
    [[0; TILE_WIDTH]; TILE_HEIGHT]
}

pub fn new_random() -> Tile {
    let mut rtn = new_empty();
    let mut rng = rand::thread_rng();
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            rtn[y][x] = rng.gen::<PaletteIndex>();
        }
    }
    rtn
}
