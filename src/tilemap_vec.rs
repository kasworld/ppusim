use image::RgbaImage;
use std::{
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

use crate::{palette, rgba, tile_vec, tilemap, tilemap_buffer};

pub const TILE_MAP_VEC_SIZE: usize = 4096;

#[derive(Debug)]
pub struct TileMapVec(pub Vec<tilemap::TileMap>);

impl TileMapVec {
    pub fn new_empty() -> Self {
        Self(vec![tilemap::TileMap::new_empty(); TILE_MAP_VEC_SIZE])
    }

    pub fn render<'a>(
        &mut self,
        dst: &'a mut RgbaImage,
        tilemapbuffer: &'a tilemap_buffer::TileMapBuffer,
        tilevec: &'a tile_vec::TileVec,
        pal: &'a palette::Palette,
    ) -> &'a mut RgbaImage {
        let mut tilemap_list = vec![0usize; 0];
        let (dstw, dsth) = (dst.width(), dst.height());
        for i in 0..TILE_MAP_VEC_SIZE {
            if self.0[i].is_in_dst(dstw as isize, dsth as isize) {
                tilemap_list.push(i);
            }
        }
        println!("drawable tilemap {}", tilemap_list.len());

        let mut max_tilemap_num_rendered = 0;
        for y in 0..dsth {
            for x in 0..dstw {
                for tm_index in &tilemap_list {
                    let tm = self.0[*tm_index];
                    let pal_index =
                        tm.get_at_dst_unchecked(x as isize, y as isize, tilemapbuffer, tilevec);
                    if pal_index == 0 {
                        continue;
                    }
                    let px = pal.get_at(tm.upper_palette_index, pal_index);
                    dst.put_pixel(x, y, px);
                    if max_tilemap_num_rendered < *tm_index {
                        max_tilemap_num_rendered = *tm_index;
                    }
                    break; // skip rendered pixel
                }
            }
        }
        println!("max rendered tilemap num {}", max_tilemap_num_rendered);
        for i in 0..tilemap_list.len() {
            if tilemap_list[i] == max_tilemap_num_rendered {
                println!("total {} tilemap rendered", i);
            }
        }
        dst
    }

    pub fn render_multi(
        mut self,
        mut dst: RgbaImage,
        tilemapbuffer: tilemap_buffer::TileMapBuffer,
        tilevec: tile_vec::TileVec,
        pal: palette::Palette,
    ) -> RgbaImage {
        let mut tilemap_list = vec![0usize; 0];
        let (dstw, dsth) = (dst.width(), dst.height());
        for i in 0..TILE_MAP_VEC_SIZE {
            if self.0[i].is_in_dst(dstw as isize, dsth as isize) {
                tilemap_list.push(i);
            }
        }
        println!("drawable tilemap {}", tilemap_list.len());

        let worker_count = get_thread_count();
        let (tx, rx) = mpsc::channel();
        let mut handles = Vec::new();
        let tm_vec = Arc::new(self.0.clone());
        let tm_list = Arc::new(tilemap_list);
        let tl_vec = Arc::new(tilevec);
        let tl_m_buf = Arc::new(tilemapbuffer);
        let pal = Arc::new(pal);
        for wid in 0..worker_count {
            let tx1 = tx.clone();
            let tm_vec = tm_vec.clone();
            let tm_list = tm_list.clone();
            let tl_vec = tl_vec.clone();
            let tl_m_buf = tl_m_buf.clone();
            let pal = pal.clone();
            let h = thread::spawn(move || {
                Self::worker(
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
        dst
    }

    fn worker(
        tm_vec: &[tilemap::TileMap],
        wid: u32,    // this worker id
        wnum: usize, // total worker count
        tilemap_list: &Vec<usize>,
        tilevec: &tile_vec::TileVec,
        tilemapbuffer: &tilemap_buffer::TileMapBuffer,
        pal: &palette::Palette,
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
}

pub fn get_thread_count() -> usize {
    let count = thread::available_parallelism().unwrap().get();
    assert!(count >= 1_usize);
    return count;
}
