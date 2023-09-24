use image::RgbaImage;
use std::{
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

use crate::{
    palette::Palette, rgba, tile_vec::TileVec, tilemap::TileMap, tilemap_buffer::TileMapBuffer,
};

pub const TILE_MAP_VEC_SIZE: usize = 4096;

#[derive(Debug)]
pub struct TileMapVec(pub Vec<TileMap>);

impl TileMapVec {
    pub fn new_empty() -> Self {
        Self(vec![TileMap::new_empty(); TILE_MAP_VEC_SIZE])
    }

    pub fn render_multi(
        mut self,
        worker_count: usize,
        mut dst: RgbaImage,
        tilemapbuffer: TileMapBuffer,
        tilevec: TileVec,
        pal: Palette,
    ) -> (Self, RgbaImage, TileMapBuffer, TileVec, Palette) {
        let mut tilemap_list = vec![0usize; 0];
        let (dstw, dsth) = (dst.width(), dst.height());
        for i in 0..TILE_MAP_VEC_SIZE {
            if self.0[i].is_in_dst(dstw as isize, dsth as isize) {
                tilemap_list.push(i);
            }
        }
        println!("drawable tilemap {} worker {worker_count}", tilemap_list.len());

        let (tx, rx) = mpsc::channel();
        let mut handles = Vec::new();
        let tm_vec = Arc::new(self.0.clone());
        let tm_list = Arc::new(tilemap_list.clone());
        let tl_vec = Arc::new(tilevec.clone());
        let tl_m_buf = Arc::new(tilemapbuffer.clone());
        let pale = Arc::new(pal.clone());
        for wid in 0..worker_count {
            let tx1 = tx.clone();
            let tm_vec = tm_vec.clone();
            let tm_list = tm_list.clone();
            let tl_vec = tl_vec.clone();
            let tl_m_buf = tl_m_buf.clone();
            let pal = pale.clone();
            let h = thread::spawn(move || {
                worker(
                    &tm_vec,
                    wid as u32,
                    worker_count,
                    &tm_list,
                    &tl_vec,
                    &tl_m_buf,
                    &pal,
                    dstw,
                    dsth,
                    tx1,
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
        (self, dst, tilemapbuffer, tilevec, pal)
    }
}

fn worker(
    tm_vec: &[TileMap],
    wid: u32,    // this worker id
    wnum: usize, // total worker count
    tilemap_list: &Vec<usize>,
    tilevec: &TileVec,
    tilemapbuffer: &TileMapBuffer,
    pal: &Palette,
    w: u32,
    h: u32,
    tx: Sender<(u32, u32, rgba::RGBA)>,
) {
    for y in (wid..h).step_by(wnum) {
        for x in 0..w {
            for tm_index in tilemap_list {
                let tm = tm_vec[*tm_index];
                let pal_index =
                    tm.get_at_dst_unchecked(x as isize, y as isize, tilemapbuffer, tilevec);
                if pal_index == 0 {
                    continue;
                }
                let px = pal.get_at(tm.upper_palette_index, pal_index);
                tx.send((x, y, px)).unwrap();
                break; // skip rendered pixel
            }
        }
    }
}
