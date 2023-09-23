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

    // calculated in is_in_dst
    px_w: isize,
    px_h: isize,
    scaled_w: isize,
    scaled_h: isize,
    end_x: isize,
    end_y: isize,
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

            // call is_in_dst to set
            px_w: 0,
            px_h: 0,
            scaled_w: 0,
            scaled_h: 0,
            end_x: 0,
            end_y: 0,
        }
    }

    pub fn calc_area(self) -> usize {
        self.wh.0 as usize * self.wh.1 as usize
    }

    fn is_enbaled(self) -> bool {
        self.enable && self.scale.0 != 0 && self.scale.1 != 0
    }

    pub fn is_in_dst(&mut self, dstw: usize, dsth: usize) -> bool {
        self.px_w = self.wh.0 as isize * tile::TILE_WIDTH as isize;
        self.px_h = self.wh.1 as isize * tile::TILE_WIDTH as isize;
        self.scaled_w = self.px_w * (self.scale.0.abs() as isize);
        self.scaled_h = self.px_h * (self.scale.1.abs() as isize);
        self.end_x = self.pos.0 as isize + self.scaled_w;
        self.end_y = self.pos.1 as isize + self.scaled_h;

        if !self.is_enbaled() {
            return false;
        }
        if self.pos.0 as isize >= dstw as isize || self.pos.1 as isize >= dsth as isize {
            return false;
        }
        if self.end_x < 0 || self.end_y < 0 {
            return false;
        }
        return true;
    }

    // call precalc_to_render, is_in_dst before call
    pub fn get_at_dst_unchecked(
        self,
        dst_x: isize,
        dst_y: isize,
        tilemapbuffer: &tilemap_buffer::TileMapBuffer,
        tilevec: &tile_vec::TileVec,
    ) -> tile::PaletteIndex {
        let tm_px_x = if self.scale.0 > 0 {
            (dst_x - (self.pos.0 as isize)) / (self.scale.0 as isize)
        } else {
            (self.end_x - dst_x) / (self.scale.0.abs() as isize)
        };
        if tm_px_x < 0 || tm_px_x >= self.px_w {
            return 0;
        }

        let tm_px_y = if self.scale.1 > 0 {
            (dst_y - (self.pos.1 as isize)) / (self.scale.1 as isize)
        } else {
            (self.end_y - dst_y) / (self.scale.1.abs() as isize)
        };
        if tm_px_y < 0 || tm_px_y >= self.px_h {
            return 0;
        }
        tilevec.get_at(
            self.upper_tilevec_index,
            tilemapbuffer.get_at(
                self.tilemap_buffer_index as usize
                    + (tm_px_y as usize / tile::TILE_HEIGHT) * (self.wh.0 as usize)
                    + tm_px_x as usize / tile::TILE_WIDTH,
            ) as usize,
        )[tm_px_y as usize % tile::TILE_HEIGHT][tm_px_x as usize % tile::TILE_WIDTH]
    }
}
