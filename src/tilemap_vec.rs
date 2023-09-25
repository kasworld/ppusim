use image::RgbaImage;
use std::{
    ops::Range,
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

use crate::{
    palette::{Palette, self}, rgba, tile_vec::TileVec, tilemap::TileMap, tilemap_buffer::TileMapBuffer,
};

pub const TILE_MAP_VEC_SIZE: usize = 4096;

#[derive(Debug)]
pub struct TileMapVec(pub Vec<TileMap>);

impl TileMapVec {
    pub fn new_empty() -> Self {
        Self(vec![TileMap::new_empty(); TILE_MAP_VEC_SIZE])
    }

    pub fn render_multi<'a>(
        &'a mut self,
        worker_count: usize,
        dstw: u32,
        dsth: u32,
        tilemapbuffer: &'a TileMapBuffer,
        tilevec: &'a TileVec,
        pal: &'a Palette,
    ) -> RgbaImage {
        let mut dst = image::RgbaImage::new(dstw, dsth);
        let mut tilemap_render_list = Vec::with_capacity(TILE_MAP_VEC_SIZE);
        for i in 0..TILE_MAP_VEC_SIZE {
            if self.0[i].is_in_dst(dstw as isize, dsth as isize) {
                tilemap_render_list.push(self.0[i]);
            }
        }
        let (tx, rx) = mpsc::channel();
        let mut handles = Vec::new();
        let tilemap_render_list2 = Arc::new(tilemap_render_list);
        let tilevec2 = Arc::new(tilevec.clone());
        let tilemap_buffer2 = Arc::new(tilemapbuffer.clone());
        let pale2 = Arc::new(pal.clone());
        let workrangelen = dsth / worker_count as u32;
        for wid in 0..worker_count {
            let tx1 = tx.clone();
            let tilemap_render_list3 = Arc::clone(&tilemap_render_list2);
            let tilevec3 = Arc::clone(&tilevec2);
            let tilemap_buffer3 = Arc::clone(&tilemap_buffer2);
            let pal3 = Arc::clone(&pale2);
            let wrange = if wid != (worker_count - 1) {
                workrangelen * wid as u32..workrangelen * (wid as u32 + 1)
            } else {
                workrangelen * wid as u32..dsth
            };
            let h = thread::spawn(move || {
                worker(
                    wrange,
                    dstw,
                    tx1,
                    &tilemap_render_list3,
                    &tilevec3,
                    &tilemap_buffer3,
                    &pal3,
                )
            });
            handles.push(h);
        }
        drop(tx);
        for r in rx {
            let (x, y, px) = r;
            dst.put_pixel(x, y, px);
        }
        for h in handles {
            h.join().unwrap()
        }
        dst
    }
}

fn worker(
    wrange: Range<u32>,
    w: u32,
    tx: Sender<(u32, u32, rgba::RGBA)>,
    tilemap_render_list: &Vec<TileMap>,
    tilevec: &TileVec,
    tilemapbuffer: &TileMapBuffer,
    pal: &Palette,
) {
    for y in wrange {
        for x in 0..w {
            for tm in tilemap_render_list {
                let pal_index =
                    tm.get_at_dst_unchecked(x as isize, y as isize, tilemapbuffer, tilevec);
                if pal_index == 0 {
                    continue;
                }
                let px = palette::get_at(pal, tm.upper_palette_index, pal_index);
                tx.send((x, y, px)).unwrap();
                break; // skip rendered pixel
            }
        }
    }
}
