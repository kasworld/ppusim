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
    palette::{self, Palette},
    rgba,
    tile_vec::TileVec,
    tilemap::{self, TileMap},
    tilemap_buffer::TileMapBuffer,
};

pub const TILE_MAP_VEC_SIZE: usize = 4096;

pub type TileMapVec = Vec<TileMap>;

pub fn new_empty() -> TileMapVec {
    vec![TileMap::new_empty(); TILE_MAP_VEC_SIZE]
}

// make copy of TileMap on screen in TileMapVec
pub fn make_tilemap_render_list(tmv: &TileMapVec, dstw: u32, dsth: u32) -> TileMapVec {
    let mut tilemap_render_list = Vec::with_capacity(TILE_MAP_VEC_SIZE);
    for i in 0..TILE_MAP_VEC_SIZE {
        let mut tm = tmv[i];
        if tm.is_in_dst(dstw as isize, dsth as isize) {
            tilemap_render_list.push(tm);
        }
    }
    tilemap_render_list
}

pub fn render_multi<'a>(
    worker_count: usize,
    dstw: u32,
    dsth: u32,
    tilemap_render_list: TileMapVec,
    tilemap_buf: TileMapBuffer,
    tilevec: TileVec,
    pal: Palette,
) -> (RgbaImage, TileMapBuffer, TileVec, Palette) {
    let mut dst = image::RgbaImage::new(dstw, dsth);
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    let tilemap_render_list2 = Arc::new(tilemap_render_list);
    let tilemap_buf2 = Arc::new(tilemap_buf);
    let tilevec2 = Arc::new(tilevec);
    let pal2 = Arc::new(pal);
    let workrangelen = dsth / worker_count as u32;
    for wid in 0..worker_count {
        let tx1 = tx.clone();
        let tilemap_render_list3 = Arc::clone(&tilemap_render_list2);
        let tilevec3 = Arc::clone(&tilevec2);
        let tilemap_buf3 = Arc::clone(&tilemap_buf2);
        let pal3 = Arc::clone(&pal2);
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
                &tilemap_buf3,
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
    (
        dst,
        Arc::try_unwrap(tilemap_buf2).unwrap(),
        Arc::try_unwrap(tilevec2).unwrap(),
        Arc::try_unwrap(pal2).unwrap(),
    )
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

pub fn get_size_byte() -> usize {
    TILE_MAP_VEC_SIZE * tilemap::get_size_byte()
}
