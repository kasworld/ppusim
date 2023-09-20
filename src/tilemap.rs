use std::cmp;

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
    pub fn calc_render_vars(
        self,
        dstw: usize,
        dsth: usize,
    ) -> (usize, usize, usize, usize, usize, usize) {
        let (render_start_x, tile_start_x) = if self.pos.0 < 0 {
            (0 as usize, -self.pos.0 as usize)
        } else {
            (self.pos.0 as usize, 0 as usize)
        };
        let (render_start_y, tile_start_y) = if self.pos.1 < 0 {
            (0 as usize, -self.pos.1 as usize)
        } else {
            (self.pos.1 as usize, 0 as usize)
        };
        let render_width = cmp::min(self.wh.0 as usize - tile_start_x, dstw - render_start_x);
        let render_height = cmp::min(self.wh.1 as usize - tile_start_y, dsth - render_start_y);
        return (
            render_start_x,
            render_start_y,
            tile_start_x,
            tile_start_y,
            render_width,
            render_height,
        );
    }
    pub fn render(
        self,
        mut dst: &render_dst::RenderDst,
        tilemapbuffer: tilemap_buffer::TileMapBuffer,
        tilevec: tile_vec::TileVec,
        pal: palette::Palette,
    ) -> &render_dst::RenderDst {
        let lower_palette = pal.get_lower(self.upper_palette_index);
        let lower_tilevec = tilevec.get_lower(self.upper_tilevec_index);

        let tmw = self.wh.0 as usize;
        let tmh = self.wh.1 as usize;
        let tilemapbuff = tilemapbuffer.get_buffer(self.tilemap_buffer_index as usize, tmw, tmh);

        let (
            render_start_x,
            render_start_y,
            tile_start_x,
            tile_start_y,
            render_width,
            render_height,
        ) = self.calc_render_vars(dst.w, dst.h);

        for tmy in 0..tmh {
            for tmx in 0..tmw {
                let lower_tl_index = tilemapbuff[tmy * tmw + tmx] as usize;
                let tl = lower_tilevec[lower_tl_index];
                let rgbatile = tile::palette2rgba(tl, lower_palette);
            }
        }
        dst
    }
}
