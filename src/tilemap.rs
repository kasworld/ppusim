use crate::{
    tile::{self},
    tile_vec,
    tilemap_buffer::{self},
};

// sprite or background plane
#[derive(Clone, Copy, Debug)]
pub struct TileMap {
    pub enable: bool,
    pub flip_x: bool,
    pub flip_y: bool,
    pub x: i16,
    pub y: i16,
    pub w: u8,
    pub h: u8,
    pub scale_x: u8,
    pub scale_y: u8,
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
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            scale_x: 1,
            scale_y: 1,
            flip_x: false,
            flip_y: false,
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
        self.w as usize * self.h as usize
    }

    fn is_enbaled(self) -> bool {
        self.enable && self.scale_x != 0 && self.scale_y != 0
    }

    pub fn is_in_dst(&mut self, dstw: isize, dsth: isize) -> bool {
        self.px_w = self.w as isize * tile::TILE_WIDTH as isize;
        self.px_h = self.h as isize * tile::TILE_WIDTH as isize;
        self.scaled_w = self.px_w * (self.scale_x as isize);
        self.scaled_h = self.px_h * (self.scale_y as isize);
        self.end_x = self.x as isize + self.scaled_w;
        self.end_y = self.y as isize + self.scaled_h;

        if !self.is_enbaled() {
            return false;
        }
        if self.x as isize >= dstw || self.y as isize >= dsth {
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
        let tm_px_x = if self.flip_x == false {
            (dst_x - (self.x as isize)) / (self.scale_x as isize)
        } else {
            (self.end_x - dst_x) / (self.scale_x as isize)
        };
        if tm_px_x < 0 || tm_px_x >= self.px_w {
            return 0;
        }

        let tm_px_y = if self.flip_y == false {
            (dst_y - (self.y as isize)) / (self.scale_y as isize)
        } else {
            (self.end_y - dst_y) / (self.scale_y as isize)
        };
        if tm_px_y < 0 || tm_px_y >= self.px_h {
            return 0;
        }
        tilevec.get_at_tlxy(
            self.upper_tilevec_index,
            tilemapbuffer.get_at(
                self.tilemap_buffer_index as usize
                    + (tm_px_y as usize / tile::TILE_HEIGHT) * (self.w as usize)
                    + tm_px_x as usize / tile::TILE_WIDTH,
            ) as usize,
            tm_px_x as usize % tile::TILE_WIDTH,
            tm_px_y as usize % tile::TILE_HEIGHT,
        )
    }
}
