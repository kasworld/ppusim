
#[derive(Clone, Copy, Debug)]
pub struct RGBA(u8, u8, u8, u8);
pub const UPPER_PALETTE_SIZE: usize = 256;
pub const LOWER_PALETTE_SIZE: usize = 256;
pub const PALETTE_SIZE: usize = UPPER_PALETTE_SIZE * LOWER_PALETTE_SIZE;
#[derive(Debug)]
pub struct Palette(Vec<RGBA>);

impl Palette {
    pub fn new() -> Self {
        Self(vec![RGBA(0, 0, 0, 0); PALETTE_SIZE])
    }
    pub fn get_lower<'a>(&'a self, hi: u8) -> &'a [RGBA] {
        let start = hi as usize * LOWER_PALETTE_SIZE;
        &self.0[start..start + LOWER_PALETTE_SIZE]
    }
}

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
    pub fn new() -> Self {
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
    pub fn render(self, mut dst :&[RGBA], tilemapbuffer :TileMapBuffer, palete :Palette)->&[RGBA]{
        dst
    }
}
pub const TILE_MAP_VEC_SIZE: usize = 4096;
#[derive(Debug)]
pub struct TileMapVec(Vec<TileMap>);

impl TileMapVec {
    pub fn new() -> Self {
        Self(vec![TileMap::new(); TILE_MAP_VEC_SIZE])
    }
}

#[derive(Debug)]
pub struct PPU {
    pub palette: Palette,
    pub tile_def: TileVec,
    pub tile_map_def: TileMapVec,
    pub tile_map_buffer: TileMapBuffer,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            palette: Palette::new(),
            tile_def: TileVec::new(),
            tile_map_def: TileMapVec::new(),
            tile_map_buffer: TileMapBuffer::new(),
        }
    }
}
