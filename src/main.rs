use image;
use rand::Rng;
use std::{
    env,
    ops::{BitAnd, Shl},
    thread,
    time::Instant,
};

use ppusim::{
    palette::{self, Palette},
    tile,
    tile_vec::{self, TileVec},
    tilemap::TileMap,
    tilemap_buffer::{self, TileMapBuffer},
    tilemap_vec::{self, TileMapVec},
};

const DSTW: usize = 1920;
const DSTH: usize = 1080;

fn main() {
    let mut args = env::args();
    let prgname = args.next().unwrap();

    let mut tile_map_def = new_tiledef_cover_tilemap_vec();
    let mut render_loop = false;
    let mut worker_count = get_thread_count();

    for arg in args {
        match arg.trim() {
            "tilemap_random" => {
                tile_map_def = new_random_tilemap_vec(DSTW, DSTH);
            }
            "tilemap_cover" => {
                tile_map_def = new_tiledef_cover_tilemap_vec();
            }
            "loop" => {
                render_loop = true;
            }
            "noloop" => {
                render_loop = false;
            }
            _ => {
                help(&prgname);
            }
        }
    }

    let pal = new_rainbow_palette();
    // let palette = Palette::load_from_file("palette.bmp".to_owned());
    // palette.save_to_file("palette2.bmp".to_owned());

    let tile_def = TileVec::load_from_file("tilesdef.bmp".to_string());
    // tile_def.save_to_file("tilesdef2.bmp".to_string());

    let tile_map_buffer = new_seq_tilemapbuffer();

    if render_loop {
        worker_count *= 2;
    }
    loop {
        let begin = Instant::now();
        let dst = tile_map_def.render_multi(
            worker_count,
            DSTW as u32,
            DSTH as u32,
            &tile_map_buffer,
            &tile_def,
            &pal,
        );
        print!(
            "render {}x{} worker {worker_count}, {} sec",
            DSTW,
            DSTH,
            begin.elapsed().as_secs_f64()
        );
        dst.save("ppu.bmp").unwrap();
        println!(", save {} sec", begin.elapsed().as_secs_f64());
        if render_loop == false {
            break;
        }
        worker_count -= 1;
        if worker_count <= 0 {
            break;
        }
    }
}

fn help(prgname: &String) {
    println!("{prgname} PPUSIM Pixel Processing Unit SIMulator");
    println!("args");
    println!("  tilemap_random");
    println!("  tilemap_cover");
    println!("  loop");
    println!("  noloop");
}

pub fn get_thread_count() -> usize {
    let count = thread::available_parallelism().unwrap().get();
    assert!(count >= 1_usize);
    return count;
}

pub fn new_random_tilemap_vec(dst_w: usize, dst_h: usize) -> TileMapVec {
    let mut offset: usize = 0;
    let mut rtn = TileMapVec::new_empty();
    for i in 0..tilemap_vec::TILE_MAP_VEC_SIZE {
        rtn.0[i] = new_random2_tilemap(i, offset, dst_w, dst_h);
        offset += rtn.0[i].calc_area();
    }
    println!(
        "total tile use {} / buf {} = {}",
        offset,
        tilemap_buffer::TILE_MAP_BUFFER_SIZE,
        offset as f64 / tilemap_buffer::TILE_MAP_BUFFER_SIZE as f64
    );
    rtn
}

pub fn new_random2_tilemap(
    tilemap_index: usize,
    mut offset: usize,
    dst_w: usize,
    dst_h: usize,
) -> TileMap {
    let mut rng = rand::thread_rng();
    let mut rtn = TileMap::new_empty();
    rtn.enable = true;

    let range_end = match tilemap_index {
        0..=1023 => 3,
        1024..=2047 => 15,
        2048..=3071 => 63,
        3072..=4095 => 255,
        _ => {
            panic!("out of range {}", tilemap_index)
        }
    };
    rtn.w = rng.gen_range(1..=range_end);
    rtn.h = rng.gen_range(1..=range_end);
    rtn.x = rng.gen_range(-(dst_w as i16)..dst_w as i16);
    rtn.y = rng.gen_range(-(dst_h as i16)..dst_h as i16);

    rtn.scale_x = rng.gen_range(1..=4);
    rtn.scale_y = rng.gen_range(1..=4);
    rtn.flip_x = rng.gen_bool(0.5);
    rtn.flip_y = rng.gen_bool(0.5);

    rtn.upper_palette_index = (tilemap_index % palette::UPPER_PALETTE_SIZE) as u8;
    rtn.upper_tilevec_index = (tilemap_index % tile_vec::UPPER_TILE_VEC_SIZE) as u8;

    offset = offset % tilemap_buffer::TILE_MAP_BUFFER_SIZE;
    if offset + rtn.calc_area() >= tilemap_buffer::TILE_MAP_BUFFER_SIZE {
        offset = 0;
    }
    rtn.tilemap_buffer_index = offset as u32;

    rtn
}

pub fn new_tiledef_cover_tilemap_vec() -> TileMapVec {
    let mut rtn = TileMapVec::new_empty();
    // rtn.0[0] = new_tiledef_cover_tilemap(16 + 1);
    for i in 0..256 {
        rtn.0[i] = new_tiledef_cover_tilemap(i as u8);
    }
    rtn
}

pub fn new_tiledef_cover_tilemap(tilevec_page: u8) -> TileMap {
    let mut rtn = TileMap::new_empty();
    rtn.enable = true;
    rtn.w = tile_vec::SQRT_LOWER_TILE_VEC_SIZE as u8; // cover full sub tilevec page
    rtn.h = tile_vec::SQRT_UPPER_TILE_VEC_SIZE as u8; // cover full sub tilevec page
    rtn.x = ((tilevec_page as usize % tile_vec::SQRT_LOWER_TILE_VEC_SIZE)
        * tile::TILE_WIDTH
        * tile_vec::SQRT_LOWER_TILE_VEC_SIZE) as i16;
    rtn.y = ((tilevec_page as usize / tile_vec::SQRT_UPPER_TILE_VEC_SIZE)
        * tile::TILE_HEIGHT
        * tile_vec::SQRT_UPPER_TILE_VEC_SIZE) as i16;
    rtn.scale_x = 1;
    rtn.scale_y = 1;
    rtn.flip_x = false;
    rtn.flip_y = false;
    rtn.upper_palette_index = tilevec_page;
    rtn.upper_tilevec_index = tilevec_page;
    rtn.tilemap_buffer_index = tilevec_page as u32 * rtn.calc_area() as u32;
    rtn
}

// R:5bit, G:6bit, B:5bit
pub fn new_rainbow_palette() -> Palette {
    let mut rtn = Palette::new_empty();
    for i in 0..palette::PALETTE_SIZE {
        rtn.0[i] = image::Rgba([
            i.bitand(0x1f).shl(3) as u8,
            (i >> 5).bitand(0x3f).shl(2) as u8,
            (i >> 11).bitand(0x1f).shl(3) as u8,
            0xff,
        ]);
    }
    rtn
}

pub fn new_seq_tilemapbuffer() -> TileMapBuffer {
    let mut rtn = tilemap_buffer::new_empty();
    for i in 0..tilemap_buffer::TILE_MAP_BUFFER_SIZE {
        rtn[i] = (i % 256) as tilemap_buffer::TileVecIndex;
    }
    rtn
}

pub fn new_random_tile() -> tile::Tile {
    let mut rtn = tile::new_empty();
    let mut rng = rand::thread_rng();
    for y in 0..tile::TILE_HEIGHT {
        for x in 0..tile::TILE_WIDTH {
            rtn[y][x] = rng.gen::<tile::PaletteIndex>();
        }
    }
    rtn
}
