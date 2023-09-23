use image;
use rand::Rng;
use std::time::Instant;

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
    let palette = Palette::new_rainbow();
    // let palette = Palette::load_from_file("palette.bmp".to_owned());
    // palette.save_to_file("palette2.bmp".to_owned());
    let tile_def = TileVec::load_from_file("tilesdef.bmp".to_string());
    // tile_def.save_to_file("tilesdef2.bmp".to_string());

    let tile_map_def = new_tiledef_cover_tilemap_vec();
    // let tile_map_def = new_random(DSTW, DSTH);
    let tile_map_buffer = TileMapBuffer::new_seq();

    loop {
        let begin = Instant::now();
        let mut dst = &mut image::RgbaImage::new(DSTW as u32, DSTH as u32);
        dst = tile_map_def.render(dst, &tile_map_buffer, &tile_def, &palette);
        print!("render {} ", begin.elapsed().as_secs_f64());
        _ = dst;
        dst.save("ppu.bmp").unwrap();
        println!("save {}", begin.elapsed().as_secs_f64());
        break;
    }
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

pub fn new_tiledef_cover_tilemap_vec() -> TileMapVec {
    let mut rtn = TileMapVec::new_empty();
    rtn.0[0] = new_tiledef_cover_tilemap(16 + 1);
    // for i in 0..256 {
    //     rtn.0[i] = tilemap::TileMap::new_tiledef_cover(i as u8);
    // }
    rtn
}

pub fn new_random_tilemap(dst_w: usize, dst_h: usize) -> TileMap {
    let mut rng = rand::thread_rng();
    let mut rtn = TileMap::new_empty();
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

pub fn new_random2_tilemap(
    tilemap_index: usize,
    mut offset: usize,
    dst_w: usize,
    dst_h: usize,
) -> TileMap {
    let mut rng = rand::thread_rng();
    let mut rtn = TileMap::new_empty();
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

pub fn new_tiledef_cover_tilemap(tilevec_page: u8) -> TileMap {
    let mut rtn = TileMap::new_empty();
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
