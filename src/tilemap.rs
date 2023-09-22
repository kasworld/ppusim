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
    pub rotate: (u8, u8, u8),
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
            rotate: (0, 0, 0),
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
            0..=255 => ((16..17), (16..17)), // cover all tile def
            256..=1023 => ((1..4), (1..4)),
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
        rtn.upper_palette_index = (tilemap_index % (palette::UPPER_PALETTE_SIZE as usize)) as u8;
        rtn.upper_tilevec_index = (tilemap_index % (tile_vec::UPPER_TILE_VEC_SIZE)) as u8;

        offset = offset % tilemap_buffer::TILE_MAP_BUFFER_SIZE;
        if offset + rtn.calc_area() >= tilemap_buffer::TILE_MAP_BUFFER_SIZE {
            offset = 0;
        }
        rtn.tilemap_buffer_index = offset as u32;

        rtn
    }

    pub fn get_at_dst(
        self,
        dst_x: usize,
        dst_y: usize,
        tilemapbuffer: &tilemap_buffer::TileMapBuffer,
        tilevec: &tile_vec::TileVec,
    ) -> tile::PaletteIndex {
        let tlmap_w = self.wh.0 as usize;
        let tm_x = dst_x - self.pos.0 as usize;
        if tm_x >= tlmap_w * tile::TILE_WIDTH {
            return 0;
        }

        let tlmap_h = self.wh.1 as usize;
        let tm_y = dst_y - self.pos.1 as usize;
        if tm_y >= tlmap_h * tile::TILE_HEIGHT {
            return 0;
        }

        tilevec.get_at(
            self.upper_tilevec_index,
            tilemapbuffer.get_at(
                self.tilemap_buffer_index as usize
                    + (tm_y / tile::TILE_HEIGHT) * tlmap_w
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
