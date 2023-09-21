use rand::Rng;

use crate::{palette, render_dst, rgba::RGBA, tile, tile_vec, tilemap_buffer};

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

        rtn.wh = (tw, th);
        rtn.pos = (0, 0);
        // rtn.wh = (rng.gen_range(0..tw), rng.gen_range(0..th));
        // rtn.pos = (
        //     rng.gen_range(-(dst_w as i16)..dst_w as i16),
        //     rng.gen_range(-(dst_h as i16)..dst_h as i16),
        // );
        rtn.upper_palette_index = rng.gen::<u8>();
        rtn.upper_tilevec_index = rng.gen::<u8>();
        rtn.tilemap_buffer_index = rng.gen_range(
            0..tilemap_buffer::TILE_MAP_BUFFER_SIZE - rtn.wh.0 as usize * rtn.wh.1 as usize,
        ) as u32;

        rtn
    }

    pub fn get_rbga_at_dst(
        self,
        dst_x: usize,
        dst_y: usize,
        tilemapbuffer: &tilemap_buffer::TileMapBuffer,
        tilevec: &tile_vec::TileVec,
        pal: &palette::Palette,
    ) -> Option<RGBA> {
        let tlmap_w = self.wh.0 as usize;
        let tm_x = dst_x - self.pos.0 as usize;
        if tm_x >= tlmap_w * tile::TILE_WIDTH {
            return Option::None;
        }

        let tlmap_h = self.wh.1 as usize;
        let tm_y = dst_y - self.pos.1 as usize;
        if tm_y >= tlmap_h * tile::TILE_HEIGHT {
            return Option::None;
        }

        let lower_palette = pal.get_lower(self.upper_palette_index);
        let lower_tilevec = tilevec.get_lower(self.upper_tilevec_index);
        let tilemapbuff =
            tilemapbuffer.get_buffer(self.tilemap_buffer_index as usize, tlmap_w, tlmap_h);

        // tm_x, tm_y point in tilemap pixel
        // let tly = tm_y / tile::TILE_HEIGHT;
        // let tly_d = tm_y % tile::TILE_HEIGHT;
        // let tlx = tm_x / tile::TILE_WIDTH;
        // let tlx_d = tm_x % tile::TILE_WIDTH;
        // let tilemap_base = tly * tlmap_w;
        // let lower_tl_index = tilemapbuff[tilemap_base + tlx] as usize;
        // let tl = lower_tilevec[lower_tl_index];
        // return Option::Some(lower_palette[tl[tly_d][tlx_d] as usize]);

        return Option::Some(
            lower_palette[lower_tilevec[tilemapbuff
                [(tm_y / tile::TILE_HEIGHT) * tlmap_w + tm_x / tile::TILE_WIDTH]
                as usize][tm_y % tile::TILE_HEIGHT][tm_x % tile::TILE_WIDTH]
                as usize],
        );
    }

    pub fn calc_render_count(self, dstw: usize, dsth: usize) -> usize {
        if !self.enable {
            return 0;
        }
        let (_, _, _, _, render_width, render_height) = self.calc_render_vars(dstw, dsth);
        if render_width <= 0 || render_height <= 0 {
            // out of screen
            return 0;
        }
        render_width as usize * render_height as usize
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
        if !self.enable {
            return dst;
        }
        let (
            render_start_x,
            render_start_y,
            tile_start_x,
            tile_start_y,
            render_width,
            render_height,
        ) = self.calc_render_vars(dst.w, dst.h);
        if render_width <= 0 || render_height <= 0 {
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
            let tly_cur = tile_start_y + y;
            let tly = tly_cur / tile::TILE_HEIGHT;
            let tly_d = tly_cur % tile::TILE_HEIGHT;
            let dst_bufbase = dst.w * (render_start_y + y);
            let tilemap_base = tly * tlmap_w;

            for x in 0..render_width as usize {
                let tlx_cur = tile_start_x + x;
                // let tlx = tlx_cur / tile::TILE_WIDTH;
                // let tlx_d = tlx_cur % tile::TILE_WIDTH;
                // let rnd_x = render_start_x + x;
                // let lower_tl_index = tilemapbuff[tilemap_base + tlx] as usize;
                // let tl = lower_tilevec[lower_tl_index];
                // dst.buffer[dst_bufbase + rnd_x] = lower_palette[tl[tly_d][tlx_d] as usize];
                // optimized
                dst.buffer[dst_bufbase + render_start_x + x] = lower_palette[lower_tilevec
                    [tilemapbuff[tilemap_base + tlx_cur / tile::TILE_WIDTH] as usize][tly_d]
                    [tlx_cur % tile::TILE_WIDTH]
                    as usize];
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
