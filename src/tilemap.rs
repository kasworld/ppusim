use rand::Rng;

use crate::{palette, render_dst, tile, tile_vec, tilemap_buffer};

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
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let mut rtn = Self::new_empty();
        rtn.enable = true;
        rtn.pos = (0,0);
        rtn.wh = (240,135);
        // rtn.pos = (rng.gen::<i16>(), rng.gen::<i16>());
        // rtn.wh = (rng.gen::<u8>(), rng.gen::<u8>());
        rtn.upper_palette_index = rng.gen::<u8>();
        rtn.upper_tilevec_index = rng.gen::<u8>();
        rtn.tilemap_buffer_index = rng.gen_range(
            0..tilemap_buffer::TILE_MAP_BUFFER_SIZE - rtn.wh.0 as usize * rtn.wh.1 as usize,
        ) as u32;

        rtn
    }

    // render_width render_height can negative if out of screen
    pub fn calc_render_vars(
        self,
        dstw: usize,
        dsth: usize,
    ) -> (usize, usize, usize, usize, isize, isize) {
        let (render_start_x, tile_start_x) = if self.pos.0 < 0 {
            (0 as usize, -(self.pos.0 as isize) as usize)
        } else {
            (self.pos.0 as usize, 0 as usize)
        };
        let (render_start_y, tile_start_y) = if self.pos.1 < 0 {
            (0 as usize, -(self.pos.1 as isize) as usize)
        } else {
            (self.pos.1 as usize, 0 as usize)
        };
        let render_width = min(
            self.wh.0 as isize * tile::TILE_WIDTH as isize - tile_start_x as isize,
            dstw as isize - render_start_x as isize,
        );
        let render_height = min(
            self.wh.1 as isize * tile::TILE_HEIGHT as isize - tile_start_y as isize,
            dsth as isize - render_start_y as isize,
        );
        return (
            render_start_x,
            render_start_y,
            tile_start_x,
            tile_start_y,
            render_width,
            render_height,
        );
    }
    pub fn render<'a>(
        self,
        dst: &'a mut render_dst::RenderDst,
        tilemapbuffer: &'a tilemap_buffer::TileMapBuffer,
        tilevec: &'a tile_vec::TileVec,
        pal: &'a palette::Palette,
    ) -> &'a mut render_dst::RenderDst {
        let (
            render_start_x,
            render_start_y,
            tile_start_x,
            tile_start_y,
            render_width,
            render_height,
        ) = self.calc_render_vars(dst.w, dst.h);
        if !self.enable || render_width <= 0 || render_height <= 0 {
            // out of screen
            return dst;
        }

        let lower_palette = pal.get_lower(self.upper_palette_index);
        let lower_tilevec = tilevec.get_lower(self.upper_tilevec_index);
        let tlmap_w = self.wh.0 as usize;
        let tlmap_h = self.wh.1 as usize;
        let tilemapbuff =
            tilemapbuffer.get_buffer(self.tilemap_buffer_index as usize, tlmap_w, tlmap_h);

        for y in 0..render_height as usize {
            let tly = (tile_start_y + y) / tile::TILE_HEIGHT;
            let tly_d = (tile_start_y + y) % tile::TILE_HEIGHT;
            let rnd_y = render_start_y + y;

            for x in 0..render_width as usize {
                let tlx = (tile_start_x + x) / tile::TILE_WIDTH;
                let tlx_d = (tile_start_x + x) % tile::TILE_WIDTH;
                let rnd_x = render_start_x + x;

                let lower_tl_index = tilemapbuff[tly * tlmap_w + tlx] as usize;
                let tl = lower_tilevec[lower_tl_index];

                dst.buffer[dst.w * rnd_y + rnd_x] = lower_palette[tl[tlx_d][tly_d] as usize];
            }
        }
        dst
    }
}

pub fn min(v1: isize, v2: isize) -> isize {
    if v1 < v2 {
        v1
    } else {
        v2
    }
}
