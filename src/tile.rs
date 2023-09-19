use crate::palette;
use crate::rgba;

// hold lower palette index
pub type PaletteIndex = u8;

pub const TILE_WIDTH: usize = 8;

pub const TILE_HEIGHT: usize = 8;

pub type Tile = [[PaletteIndex; TILE_WIDTH]; TILE_HEIGHT];

pub fn new_empty() -> Tile {
    [[0; TILE_WIDTH]; TILE_HEIGHT]
}

pub fn palette2rgba_at(tl: Tile, pal: &[rgba::RGBA] , x :usize, y:usize) ->rgba::RGBA{
    let i = tl[x][y] as usize;
    pal[i]
}

pub fn palette2rgba(tl: Tile, pal: &[rgba::RGBA]) -> [[rgba::RGBA; TILE_WIDTH]; TILE_HEIGHT] {
    let mut rtn = [[rgba::RGBA::new_empty(); TILE_WIDTH]; TILE_HEIGHT];
    for x in 0..TILE_HEIGHT {
        for y in 0..TILE_WIDTH {
            let i = tl[x][y] as usize;
            rtn[x][y] = pal[i];
        }
    }
    rtn
}
