use rand::Rng;

use crate::{
    palette::{self},
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

    pub fn new_random(dst_w: usize, dst_h: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut rtn = Self::new_empty();
        rtn.enable = true;
        let tw = (dst_w / tile::TILE_WIDTH) as u8;
        let th = (dst_h / tile::TILE_HEIGHT) as u8;

        // rtn.wh = (tw, th);
        // rtn.pos = (0, 0);
        rtn.wh = (rng.gen_range(0..tw), rng.gen_range(0..th));
        rtn.pos = (
            rng.gen_range(-(dst_w as i16)..dst_w as i16),
            rng.gen_range(-(dst_h as i16)..dst_h as i16),
        );
        rtn.upper_palette_index = rng.gen::<u8>();
        rtn.upper_tilevec_index = rng.gen::<u8>();
        rtn.tilemap_buffer_index =
            rng.gen_range(0..tilemap_buffer::TILE_MAP_BUFFER_SIZE - rtn.calc_area()) as u32;

        rtn
    }

    pub fn calc_area(self) -> usize {
        self.wh.0 as usize * self.wh.1 as usize
    }
    pub fn is_enbaled(self) -> bool {
        self.enable && self.scale.0 != 0 && self.scale.1 != 0
    }
    pub fn new_random2(
        tilemap_index: usize,
        mut offset: usize,
        dst_w: usize,
        dst_h: usize,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let mut rtn = Self::new_empty();
        rtn.enable = true;
        let tw = (dst_w / tile::TILE_WIDTH) as u8;
        let th = (dst_h / tile::TILE_HEIGHT) as u8;

        let wh_range = match tilemap_index {
            0..=1023 => ((1..4), (1..4)),
            1024..=2047 => ((1..16), (1..16)),
            2048..=3071 => ((1..64), (1..64)),
            3072..=4095 => ((1..tw), (1..th)),
            _ => {
                panic!("out of range {}", tilemap_index)
            }
        };
        rtn.wh = (rng.gen_range(wh_range.0), rng.gen_range(wh_range.1));
        rtn.pos = (
            rng.gen_range(-(dst_w as i16)..dst_w as i16),
            rng.gen_range(-(dst_h as i16)..dst_h as i16),
        );

        rtn.scale = (rng.gen_range(-4..=4), rng.gen_range(-4..=4));

        rtn.upper_palette_index = (tilemap_index % palette::UPPER_PALETTE_SIZE) as u8;
        rtn.upper_tilevec_index = (tilemap_index % tile_vec::UPPER_TILE_VEC_SIZE) as u8;

        offset = offset % tilemap_buffer::TILE_MAP_BUFFER_SIZE;
        if offset + rtn.calc_area() >= tilemap_buffer::TILE_MAP_BUFFER_SIZE {
            offset = 0;
        }
        rtn.tilemap_buffer_index = offset as u32;

        rtn
    }

    pub fn new_tiledef_cover(tilevec_page: u8) -> Self {
        let mut rtn = Self::new_empty();
        rtn.enable = true;
        rtn.wh = (16, 16); // cover full sub tilevec page
        rtn.pos = (
            ((tilevec_page % 16) as usize * tile::TILE_WIDTH * 16) as i16,
            ((tilevec_page / 16) as usize * tile::TILE_HEIGHT * 16) as i16,
        );
        rtn.scale = (1, 1);
        rtn.upper_palette_index = tilevec_page;
        rtn.upper_tilevec_index = tilevec_page;
        rtn.tilemap_buffer_index = tilevec_page as u32 * rtn.calc_area() as u32;
        rtn
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
        };
        if tm_x >= (self.wh.0 as usize) * tile::TILE_WIDTH {
            return 0;
        }

        let tm_y = if self.scale.1 > 0 {
            (dst_y - self.pos.1 as usize) / self.scale.1 as usize
        } else {
            let scale = (-(self.scale.1 as isize)) as usize;
            (self.pos.1 as usize + (self.wh.1 as usize) * scale * tile::TILE_HEIGHT - dst_y) / scale
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

pub fn min(v1: isize, v2: isize) -> isize {
    if v1 < v2 {
        v1
    } else {
        v2
    }
}
