use crate::{
    tile::{self},
    tile_vec,
    tilemap_buffer::{self},
};

// sprite or background plane
#[derive(Clone, Copy, Debug)]
pub struct TileMap {
    pub enable: bool,
    pub pos: (i16, i16),
    pub wh: (u8, u8),
    pub scale: (i8, i8),
    pub upper_palette_index: u8,
    pub upper_tilevec_index: u8,
    pub tilemap_buffer_index: u32,
}

impl TileMap {
    pub fn new_empty() -> Self {
        Self {
            pos: (0, 0),
            wh: (1, 1),
            scale: (1, 1),
            upper_palette_index: 0,
            upper_tilevec_index: 0,
            tilemap_buffer_index: 0,
            enable: false,
        }
    }

    pub fn calc_area(self) -> usize {
        self.wh.0 as usize * self.wh.1 as usize
    }

    pub fn is_enbaled(self) -> bool {
        self.enable && self.scale.0 != 0 && self.scale.1 != 0
    }

    pub fn get_at_dst(
        self,
        dst_x: usize,
        dst_y: usize,
        tilemapbuffer: &tilemap_buffer::TileMapBuffer,
        tilevec: &tile_vec::TileVec,
    ) -> tile::PaletteIndex {
        if !self.is_enbaled() {
            return 0;
        }

        let tm_x = if self.scale.0 > 0 {
            (dst_x - self.pos.0 as usize) / self.scale.0 as usize
        } else {
            let scale = (-(self.scale.0 as isize)) as usize;
            (self.pos.0 as usize + (self.wh.0 as usize) * scale * tile::TILE_WIDTH - dst_x) / scale
            // ((dst_x as isize - self.pos.0 as isize) / (self.scale.0 as isize)) as usize + (self.wh.0 as usize) * tile::TILE_WIDTH
        };
        if tm_x >= (self.wh.0 as usize) * tile::TILE_WIDTH {
            return 0;
        }

        let tm_y = if self.scale.1 > 0 {
            (dst_y - self.pos.1 as usize) / self.scale.1 as usize
        } else {
            let scale = (-(self.scale.1 as isize)) as usize;
            (self.pos.1 as usize + (self.wh.1 as usize) * scale * tile::TILE_HEIGHT - dst_y) / scale
            // ((dst_y as isize - self.pos.1 as isize) / (self.scale.1 as isize)) as usize + (self.wh.1 as usize) * tile::TILE_HEIGHT
        };
        if tm_y >= (self.wh.1 as usize) * tile::TILE_HEIGHT {
            return 0;
        }

        tilevec.get_at(
            self.upper_tilevec_index,
            tilemapbuffer.get_at(
                self.tilemap_buffer_index as usize
                    + (tm_y / tile::TILE_HEIGHT) * (self.wh.0 as usize)
                    + tm_x / tile::TILE_WIDTH,
            ) as usize,
        )[tm_y % tile::TILE_HEIGHT][tm_x % tile::TILE_WIDTH]
    }
}
