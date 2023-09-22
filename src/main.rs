use image;
use std::time::Instant;

use ppusim::{
    palette::Palette, tile_vec::TileVec, tilemap_buffer::TileMapBuffer, tilemap_vec::TileMapVec,
};

const DSTW: usize = 1920;
const DSTH: usize = 1080;

fn main() {
    let palette = Palette::new_rainbow();
    // let palette = Palette::load_from_file("palette.bmp".to_owned());
    // palette.save_to_file("palette2.bmp".to_owned());
    let tile_def = TileVec::load_from_file("tilesdef.bmp".to_string());
    // tile_def.save_to_file("tilesdef2.bmp".to_string());

    let tile_map_def = TileMapVec::new_random(DSTW, DSTH);
    let tile_map_buffer = TileMapBuffer::new_seq();

    loop {
        let begin = Instant::now();
        let mut dst = &mut image::RgbaImage::new(
            DSTW as u32, DSTH as u32);
        dst = tile_map_def.render(
            dst, &tile_map_buffer, &tile_def, &palette);
        print!("render {} ", begin.elapsed().as_secs_f64());
        _ = dst;
        dst.save("ppu.bmp").unwrap();
        println!("save {}", begin.elapsed().as_secs_f64());
        break;
    }
}
