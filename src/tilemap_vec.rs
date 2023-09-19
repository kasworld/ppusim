use crate::tilemap;

pub const TILE_MAP_VEC_SIZE: usize = 4096;

#[derive(Debug)]
pub struct TileMapVec(Vec<tilemap::TileMap>);

impl TileMapVec {
    pub fn new() -> Self {
        Self(vec![tilemap::TileMap::new(); TILE_MAP_VEC_SIZE])
    }
}
